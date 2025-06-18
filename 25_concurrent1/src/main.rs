// use std::sync::mpsc;
// use std::thread::{self, JoinHandle};
// use std::time::Duration;

// fn main() {
//     thread_study();
// }

// fn thread_study() {
//     // let handle = thread::spawn(|| {
//     //     for i in 1..10 {
//     //         println!("hi number {} from the spawned thread!", i);
//     //         thread::sleep(Duration::from_millis(1));
//     //     }
//     // });
//     // for i in 1..5 {
//     //     println!("hi number {} from the main thread!", i);
//     //     thread::sleep(Duration::from_millis(1));
//     // }
//     // handle.join().unwrap();

//     // let v = vec![1, 2, 3];
//     // // error: closure may outlive the current function, but it borrows `v`, which is owned by the current function
//     // let handle = thread::spawn(move || {
//     //     println!("Here is a vector: {:?}", v);
//     // });
//     // // drop(v); // error: use of moved value: `v`
//     // handle.join().unwrap();

//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_millis(200));
//         }
//     });
//     for received in rx {
//         println!("Got: {received}");
//     }
// }

// 课后习题1：实现多线程文件处理器
use std::path::PathBuf;
use std::sync::mpsc;
use std::{fs, thread};

// 读取文件内容并打印
fn process_file(path: PathBuf) {
    // PathBuf 类型，代表文件路径
    match fs::read_to_string(&path) {
        Ok(content) => println!(
            "文件：{:?}\n内容：\n{}\n{}",
            path,
            content,
            "-".repeat(20) // 分割线
        ),
        Err(e) => eprintln!("无法读取文件 {:?}：{}", path, e),
    }
}

/*
    主线程 (生产者)
        │
        ▼
    [主通道] (同步通道，缓冲区=4) → 控制最大待处理任务数
        │
        ▼
    任务分发线程 (协调者)
        │
        ▼ (轮询分发)
    [子通道1] → 工作线程1 (消费者)
    [子通道2] → 工作线程2 (消费者)
    [子通道3] → 工作线程3 (消费者)
    [子通道4] → 工作线程4 (消费者)
*/
// 关闭顺序：
// 1. 主线程完成发送 → drop(tx) → 主通道关闭
// 2. 分发线程收完所有任务 → 循环结束 → 销毁所有 child_tx
// 3. 每个工作线程的 child_rx 接收端关闭 → 退出循环
// 4. 工作线程自然结束 → 分发线程 join → 主线程结束
fn main() {
    // 模拟包含10个文件路径的向量（file1.txt 到 file10.txt）
    let files = (1..=10)
        .map(|i| PathBuf::from(format!("file{}.txt", i)))
        .collect::<Vec<PathBuf>>();

    // 创建带缓冲区的通道 - 缓冲区大小 = 最大并发数
    // 当缓冲区满（4个任务等待）时，发送操作 (tx.send()) 会阻塞
    // 只有当工作线程从接收端取走任务后，发送操作才能继续
    // 实现任务的流量控制，防止生产者过快生产任务
    let (tx, rx) = mpsc::sync_channel(4); // 缓冲区大小 = 最大并发数

    // 创建工作线程池
    let mut workers = vec![];

    // 单个工作线程负责从通道接收任务
    let worker = thread::spawn({
        // 所有权转移，确保线程安全
        move || {
            println!("任务分发线程已启动");

            // 使用向量存储工作线程
            let mut child_workers = vec![];

            // 创建4个工作线程
            for id in 0..4 {
                // mpsc::channel() 为每个工作线程创建独立的异步通道
                // child_tx：任务分发线程使用的发送端
                // child_rx：工作线程使用的接收端
                let (child_tx, child_rx) = mpsc::channel();

                let child_worker = thread::spawn(move || {
                    println!("工作线程 {} 已启动", id);

                    // 迭代接收文件路径
                    for path in child_rx.iter() {
                        println!("工作线程 {} 正在处理：{:?}", id, path);
                        process_file(path);
                    }

                    println!("工作线程 {} 已结束", id);
                });

                // 存储所有工作线程的发送端和句柄
                child_workers.push((child_tx, child_worker));
            }

            // 轮询方式分发任务
            let mut index = 0;
            for path in rx.iter() {
                // 选择下一个工作线程
                let (child_tx, _) = &child_workers[index];

                // 发送任务到工作线程
                child_tx.send(path).expect("向工作线程发送任务失败");

                // 移动到下一个工作线程
                index = (index + 1) % child_workers.len();
            }

            // 关闭所有子通道
            for (child_tx, _) in &child_workers {
                drop(child_tx.clone());
            }

            // 等待所有工作线程完成
            for (_, child_worker) in child_workers {
                child_worker.join().expect("工作线程异常退出");
            }

            println!("任务分发线程已结束");
        }
    });

    workers.push(worker);

    // 主线程作为生产者发送文件路径
    for file in files {
        println!("主线程发送: {:?}", file);
        tx.send(file).expect("发送文件路径失败");
    }

    // 关闭发送通道
    drop(tx);

    // 等待任务分发线程完成
    for worker in workers {
        worker.join().expect("任务分发线程异常退出");
    }

    println!("所有任务已完成");
}

// 课后习题2：使用 Channel 实现程序的优雅停止
// use std::fmt;
// use std::sync::mpsc::{self, Receiver, Sender};
// use std::thread;
// use std::time::Duration;

// // 任务类型定义
// #[derive(Debug, Clone)]
// enum Task {
//     // 常规任务，包含任务ID
//     Job(i32),
//     // 停止信号，要求所有工作线程优雅退出
//     Terminate,
// }

// // 为 Task 实现自定义显示
// impl fmt::Display for Task {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Task::Job(id) => write!(f, "任务 #{}", id),
//             Task::Terminate => write!(f, "停止信号"),
//         }
//     }
// }

// fn main() {
//     // 创建4个工作线程及其通道
//     let mut worker_txs = Vec::new(); // 存储工作线程的发送端
//     let mut workers = Vec::new(); // 存储工作线程的句柄

//     // 创建停止通知通道
//     let (stop_tx, stop_rx) = mpsc::channel();

//     // 创建并启动4个工作线程
//     for id in 0..4 {
//         // 为每个工作线程创建专用通道
//         let (worker_tx, worker_rx) = mpsc::channel();
//         // worker_tx 存储在 worker_txs 中，供主线程发送任务
//         worker_txs.push(worker_tx);

//         // 为每个工作线程创建专用的停止通知发送端
//         let thread_stop_tx = stop_tx.clone();

//         let worker = thread::spawn(move || {
//             // worker_rx 传递给工作线程，用于接收任务
//             worker_thread(id, worker_rx, thread_stop_tx);
//         });

//         workers.push(worker);
//     }

//     // 创建可变的任务分发函数（闭包）
//     let mut current_worker = 0;
//     let mut send_task = |task: Task| {
//         // 添加 mut 关键字使闭包可变
//         worker_txs[current_worker].send(task).unwrap();
//         current_worker = (current_worker + 1) % worker_txs.len();
//     };

//     // 发送10个任务
//     for task_id in 1..=10 {
//         let task = Task::Job(task_id);
//         println!("[主线程] 发送任务: {}", task);
//         send_task(task); // 调用可变闭包进行分发
//     }

//     // 发送4个 Task::Terminate 停止信号
//     // 每个工作线程都会收到一个停止信号
//     println!("[主线程] 发送停止信号，等待工作线程完成当前任务...");
//     for _ in 0..4 {
//         send_task(Task::Terminate); // 调用可变闭包
//     }

//     // 等待工作线程发送停止确认
//     let mut stopped_workers = 0;
//     while stopped_workers < 4 {
//         // 主线程阻塞在 stop_rx.recv() 上
//         match stop_rx.recv() {
//             Ok(worker_id) => {
//                 println!("[主线程] 收到工作线程{}的停止确认", worker_id);
//                 stopped_workers += 1;
//             }
//             Err(_) => {
//                 // 如果通道意外关闭，跳出循环
//                 println!("[主线程] 停止通道已关闭");
//                 break;
//             }
//         }
//     }

//     // 关闭所有工作线程通道（通知工作线程退出）
//     for tx in worker_txs {
//         drop(tx); // 显式关闭通道
//     }

//     // 等待所有工作线程结束
//     for worker in workers {
//         worker.join().unwrap();
//     }

//     println!("[主线程] 所有工作线程已退出，程序结束");
// }

// /// 工作线程函数
// fn worker_thread(
//     id: u8,
//     rx: Receiver<Task>,
//     stop_tx: Sender<u8>, // 用于通知主线程本线程已停止
// ) {
//     println!("[工作线程{}] 已启动", id);

//     // 处理任务的循环
//     // 给循环命名 task_loop
//     'task_loop: for task in rx.iter() {
//         match task {
//             Task::Job(task_id) => {
//                 println!("[工作线程{}] 开始处理: {}", id, Task::Job(task_id));

//                 // 模拟任务处理
//                 let duration = Duration::from_millis(200 + (task_id as u64 % 4) * 100);
//                 thread::sleep(duration);

//                 println!("[工作线程{}] 完成处理: {}", id, Task::Job(task_id));
//             }

//             Task::Terminate => {
//                 println!("[工作线程{}] 接收到停止信号，准备退出", id);

//                 // 通知主线程本线程已收到停止信号
//                 stop_tx.send(id).unwrap();
//                 break 'task_loop;
//             }
//         }
//     }

//     // 处理通道中剩余的任务（在停止信号之后发送的任务）
//     println!("[工作线程{}] 处理剩余任务...", id);
//     for task in rx.iter() {
//         match task {
//             Task::Job(task_id) => {
//                 println!("[工作线程{}] 处理剩余任务: #{}", id, task_id);
//                 thread::sleep(Duration::from_millis(50)); // 快速处理, 使用较短的休眠时间 (50ms)
//             }
//             Task::Terminate => {
//                 // 忽略额外的停止信号，避免重复处理停止信号
//             }
//         }
//     }

//     println!("[工作线程{}] 已完成所有任务，退出", id);
// }
