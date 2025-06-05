// use std::ops::{Deref, DerefMut};

// fn main() {
//     // _box_study();

//     let x = 5;
//     let y = MyBox::new(x);
//     println!("x = {}", x);
//     // 触发解引用运算符重载
//     // 等价于 *(y.deref())（编译器自动转换）
//     println!("y = {}", *y);

//     let mut x = MyBox::new(5);
//     *x = 10;
//     println!("x = {}", *x);
// }

// // enum List {
// //     Cons(i32, Box<List>),
// //     Nil,
// // }
// // use List::{Cons, Nil};

// // trait Animal {
// //     fn speak(&self);
// // }
// // struct Dog;
// // struct Cat;
// // impl Animal for Dog {
// //     fn speak(&self) {
// //         println!("Woof!");
// //     }
// // }
// // impl Animal for Cat {
// //     fn speak(&self) {
// //         println!("Meow!");
// //     }
// // }

// // pub trait Drop {
// //     fn drop(&mut self);
// // }

// // struct Resource {
// //     name: String,
// // }
// // impl Drop for Resource {
// //     fn drop(&mut self) {
// //         println!("{} is dropped", self.name);
// //     }
// // }

// fn _box_study() {
//     // let b = Box::new(5); // 堆数据
//     // println!("b = {}", b);

//     // let s: Box<str> = "Hello, world!".into();
//     // println!("s = {}", s);
//     // let arr: Box<[i32]> = vec![1, 2, 3, 4, 5].into_boxed_slice();
//     // println!("arr = {:?}", arr);

//     // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

//     // let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
//     // for animal in animals.iter() {
//     //     animal.speak();
//     // }

//     // let _r1 = Resource {
//     //     name: "r1".to_string(),
//     // };
//     // {
//     //     let _r2 = Resource {
//     //         name: "r2".to_string(),
//     //     };
//     // }
// }

// // pub trait Deref {
// //     type Target: ?Sized;
// //     fn deref(&self) -> &Self::Target;
// // }

// struct MyBox<T>(T);
// // 构造函数实现
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
// impl<T> Deref for MyBox<T> {
//     // 关联类型
//     type Target = T;
//     fn deref(&self) -> &T {
//         println!("derefmut called");
//         // 访问元组结构体的第一个字段
//         &self.0
//     }
// }

// // pub trait DerefMut: Deref {
// //     fn deref_mut(&mut self) -> &mut Self::Target;
// // }
// impl<T> DerefMut for MyBox<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         println!("deref called");
//         // 访问元组结构体的第一个字段
//         &mut self.0
//     }
// }

// // 课后作业 1
// use std::time::Instant;

// fn main() {
//     test_stack_allocation();
//     test_heap_allocation();
// }

// fn test_stack_allocation() {
//     // 栈上分配测试 (1,000,000 个整数)
//     let n = 1_000_000;
//     // 分配
//     let start = Instant::now();
//     let mut arr: [u32; 1_000_000] = [0; 1_000_000]; // 改用u32避免负数问题
//     let duration = start.elapsed();
//     println!("分配时间: {:?}", duration); // 分配时间: 58.75µs
//     // 写入
//     let start = Instant::now();
//     for i in 0..n {
//         arr[i] = i as u32;
//     }
//     let duration = start.elapsed();
//     println!("写入时间: {:?}", duration); // 写入时间: 7.7795ms
//     // 读取和求和
//     let start = Instant::now();
//     let mut sum: u64 = 0; // 使用u64来容纳大数字
//     for i in 0..n {
//         sum += arr[i] as u64; // 将每个元素转为u64
//     }
//     let duration = start.elapsed();
//     println!("读取时间: {:?}\n", duration); // 读取时间: 7.601417ms
// }

// fn test_heap_allocation() {
//     // 堆上分配测试 (1,000,000 个整数)
//     let n = 1_000_000;
//     // 分配
//     let start = Instant::now();
//     let mut arr = Box::new([0u32; 1_000_000]); // 明确类型为u32
//     let duration = start.elapsed();
//     println!("分配时间: {:?}", duration); // 分配时间: 421.75µs
//     // 写入
//     let start = Instant::now();
//     for i in 0..n {
//         arr[i] = i as u32;
//     }
//     let duration = start.elapsed();
//     println!("写入时间: {:?}", duration); // 写入时间: 6.979958ms
//     // 读取和求和
//     let start = Instant::now();
//     let mut sum: u64 = 0;
//     for i in 0..n {
//         sum += arr[i] as u64;
//     }
//     let duration = start.elapsed();
//     println!("读取时间: {:?}", duration); // 读取时间: 7.15825ms
// }

// 课后作业 2
trait FileSystem {
    fn create_file(&mut self, name: &str) -> Result<(), String>;
    fn create_folder(&mut self, name: &str) -> Result<(), String>;
    fn list_contents(&self, indent: usize);
}

// 定义 Node 枚举，包含：文件节点 和 文件夹节点
enum Node {
    // File 和 Folder 是变体的名称
    // (FileNode) 和 (FolderNode) 是变体关联的数据类型
    File(FileNode),
    Folder(FolderNode),
}

// 文件节点结构
struct FileNode {
    name: String,
}

// 文件夹节点结构
struct FolderNode {
    name: String,
    contents: Vec<Box<Node>>, // 使用 Box<Node> 存储子节点
}

impl FileSystem for FolderNode {
    fn create_file(&mut self, name: &str) -> Result<(), String> {
        // 检查是否同名节点已存在
        // *node 对 &Box<Node> 解引用得到 Box<Node>
        // **node 对 Box<Node> 解引用得到 Node 值
        // &**Node 取得对 Node 的引用（&Node），为了避免所有权的移动
        if self.contents.iter().any(|node| match &**node {
            Node::File(f) => f.name == name,
            Node::Folder(f) => f.name == name,
        }) {
            return Err("Name already exists".to_string());
        }
        // 创建新文件
        self.contents.push(Box::new(Node::File(FileNode {
            name: name.to_string(),
        })));
        Ok(())
    }

    fn create_folder(&mut self, name: &str) -> Result<(), String> {
        // 检查是否同名节点已存在
        if self.contents.iter().any(|node| match &**node {
            Node::File(f) => f.name == name,
            Node::Folder(f) => f.name == name,
        }) {
            return Err("Name already exists".to_string());
        }
        // 创建新文件夹
        self.contents.push(Box::new(Node::Folder(FolderNode {
            name: name.to_string(),
            contents: Vec::new(),
        })));
        Ok(())
    }

    fn list_contents(&self, indent: usize) {
        // 递归列出所有内容
        // .iter() 遍历 self.contents 中的每个元素（&Box<Node> 类型）
        // .enumerate() 会将迭代器转换为新的迭代器，新迭代器产生元组 (index, item)
        for (i, node) in self.contents.iter().enumerate() {
            let is_last = i == self.contents.len() - 1;
            let prefix = if is_last { "└──" } else { "├──" };
            let item_indent = indent + 2;

            match &**node {
                // {:indent$} 功能：创建指定数量的空格缩进
                // indent$ 是一个 ​​命名参数占位符
                // indents 的值为 item_indent
                Node::File(file) => println!(
                    "{:indent$}{} {} (File)",
                    "",
                    prefix,
                    file.name,
                    indent = item_indent
                ),
                Node::Folder(folder) => {
                    // 打印子文件夹名称作为父文件夹的子项
                    println!(
                        "{:indent$}{} {} (Folder)",
                        "",
                        prefix,
                        folder.name,
                        indent = item_indent
                    );
                    folder.list_contents(item_indent + 2);
                }
            }
        }
    }
}

fn main() {
    // 创建根文件夹
    let mut root = FolderNode {
        name: "Root".to_string(),
        contents: Vec::new(),
    };

    // 在根目录添加文件和文件夹
    // 使用 .unwrap() 使得出现错误后 painc 退出程序
    root.create_file("document.txt").unwrap();
    root.create_folder("Pictures").unwrap();
    root.create_folder("Music").unwrap();

    // 在 Pictures 文件夹中添加文件
    // .iter_mut() 获取集合的​​可变引用迭代器
    // matches! 宏检查是否匹配模式（名字为 Pictures 的文件夹
    // .map() 将 &mut Box<Node> 类型转为 &mut Node 类型
    // pictures 的类型为 &mut FolderNode
    if let Some(Node::Folder(pictures)) = root
        .contents
        .iter_mut()
        .find(|n| matches!(&***n, Node::Folder(f) if f.name == "Pictures"))
        .map(|n| &mut **n)
    {
        pictures.create_file("photo1.jpg").unwrap();
        pictures.create_file("photo2.jpg").unwrap();
    }

    // 在 Music 文件夹中添加文件
    if let Some(Node::Folder(music)) = root
        .contents
        .iter_mut()
        .find(|n| matches!(&***n, Node::Folder(f) if f.name == "Music"))
        .map(|n| &mut **n)
    {
        music.create_file("song1.mp3").unwrap();
        music.create_folder("Classical").unwrap();
    }

    // 列出整个文件系统结构
    println!("File System Structure:");
    root.list_contents(0);
}
