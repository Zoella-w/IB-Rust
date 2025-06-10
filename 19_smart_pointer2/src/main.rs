// use crate::List::{Cons, Nil};
// fn main() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a)); // use of moved value: `a`
// }
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// fn main() {
//     let a = Cons(5, &(Cons(10, &(Nil))));
//     let b = Cons(3, &a);
//     let c = Cons(4, &a);
// }
// enum List<'a> {
//     Cons(i32, &'a List<'a>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;
// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("a: {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("a: {}", Rc::strong_count(&a));
//     let c = Cons(4, Rc::clone(&a));
//     println!("a: {}", Rc::strong_count(&a));
//     println!("{:?}", a);
//     println!("{:?}", b);
//     println!("{:?}", c);
// }
// #[derive(Debug)]
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use std::cell::RefCell;
// fn main() {
//     let data = RefCell::new(5);
//     // 获取不可变引用
//     {
//         let value = data.borrow();
//         println!("{}", value);
//     }
//     // 可变借用
//     {
//         let mut value = data.borrow_mut();
//         *value += 1;
//         println!("{}", value);
//     }
//     // 再次获取不可变借用
//     {
//         let value = data.borrow();
//         println!("{}", value);
//         // let mut value1 = data.borrow_mut(); // already borrowed: BorrowMutError
//         // *value1 += 1;
//         // println!("{}", value1);
//         let value1 = data.borrow();
//         println!("{}", value1);
//     }
// }

// use std::cell::RefCell;
// use std::rc::Rc;
// #[derive(Debug)]
// struct Node {
//     value: i32,
//     next: Option<Rc<RefCell<Node>>>,
// }
// fn main() {
//     let first = Rc::new(RefCell::new(Node {
//         value: 1,
//         next: None,
//     }));
//     let second = Rc::new(RefCell::new(Node {
//         value: 2,
//         next: None,
//     }));
//     // 创建引用循环
//     first.borrow_mut().next = Some(Rc::clone(&second));
//     second.borrow_mut().next = Some(Rc::clone(&first));
//     // 如果尝试打印引用计数，将看到引用循环已经发生
//     println!(
//         "first strong = {}, weak = {}",
//         Rc::strong_count(&first),
//         Rc::weak_count(&first)
//     );
//     println!(
//         "second strong = {}, weak = {}",
//         Rc::strong_count(&second),
//         Rc::weak_count(&second)
//     );
//     // println!("{:?}", &first);  // stack overflow
// }

// use std::cell::RefCell;
// use std::rc::{Rc, Weak};
// #[derive(Debug)]
// struct Node {
//     value: i32,
//     next: Option<Rc<RefCell<Node>>>,
//     prev: Option<Weak<RefCell<Node>>>, // 添加一个弱引用来指向前一个节点
// }
// fn main() {
//     let first = Rc::new(RefCell::new(Node {
//         value: 1,
//         next: None,
//         prev: None,
//     }));
//     let second = Rc::new(RefCell::new(Node {
//         value: 2,
//         next: None,
//         prev: None,
//     }));
//     // 创建非循环引用
//     first.borrow_mut().next = Some(Rc::clone(&second));
//     second.borrow_mut().prev = Some(Rc::downgrade(&first));
//     // 如果尝试打印引用计数
//     println!(
//         "first strong = {}, weak = {}",
//         Rc::strong_count(&first),
//         Rc::weak_count(&first)
//     ); // first strong = 1, weak = 1
//     println!(
//         "second strong = {}, weak = {}",
//         Rc::strong_count(&second),
//         Rc::weak_count(&second)
//     ); // second strong = 2, weak = 0
//     println!("{:?}", &first);
// }

// 课后作业

use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct User {
    name: String,
    // 一个用户拥有一个朋友列表（Vec），这个列表存储的是对其他用户的弱引用（Weak）
    // 每个朋友用户（即其他用户）需要具有内部可变性，所以每个朋友用户被包裹在 RefCell 中
    // 用户自己需要能够修改朋友列表（添加朋友），所以整个列表被包裹在 RefCell 中以提供内部可变性
    friends: RefCell<Vec<Weak<RefCell<User>>>>,
}

impl User {
    // 用户对象需要：共享所有权（Rc）、提供内部可变性（RefCell）
    fn new(name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(User {
            name: name.to_string(),
            friends: RefCell::new(Vec::new()),
        }))
    }

    // 添加好友：在 user1 和 user2 之间建立好友关系
    fn add_friend(user1: &Rc<RefCell<Self>>, user2: &Rc<RefCell<Self>>) {
        // 在 user1 的朋友列表中添加 user2 的弱引用
        user1
            .borrow_mut() // 获取 user1 的可变借用
            .friends
            .borrow_mut() // 获取 user1 好友列表的可变借用
            .push(Rc::downgrade(user2)); // 添加 user2 的弱引用
        // 在 user2 的朋友列表中添加 user1 的弱引用
        user2
            .borrow_mut()
            .friends
            .borrow_mut()
            .push(Rc::downgrade((user1)));
    }

    // 显示用户的好友列表
    fn show_friends(&self) {
        println!("{} 的朋友:", self.name);
        // 遍历好友列表中的每个弱引用
        for (i, friend) in self.friends.borrow().iter().enumerate() {
            // 将弱引用升级为强引用
            if let Some(friend_rc) = friend.upgrade() {
                let friend_ref = friend_rc.borrow();
                // 打印朋友名称
                println!("  {}. {}", i + 1, friend_ref.name);
            }
            // 朋友已释放（异常情况）
            else {
                println!("  {}. [已移除的朋友]", i + 1);
            }
        }
        println!(); // 添加空行分隔
    }
}

fn main() {
    // 创建用户 Alice
    let alice = User::new("Alice");
    println!(
        "[创建] Alice, 初始强引用计数 = {}",
        Rc::strong_count(&alice)
    );
    // 创建用户 Bob
    let bob = User::new("Bob");
    println!("[创建] Bob, 初始强引用计数 = {}", Rc::strong_count(&bob));
    // 创建用户 Charlie
    let charlie = User::new("Charlie");
    println!(
        "[创建] Charlie, 初始强引用计数 = {}",
        Rc::strong_count(&charlie)
    );
    println!("--------------------------------");

    // 建立朋友关系
    // 添加 Alice 和 Bob 为朋友
    User::add_friend(&alice, &bob);
    println!("[添加好友] Alice 和 Bob");
    println!(
        "  Alice 强引用计数 = {}, 弱引用计数 = {}",
        Rc::strong_count(&alice),
        Rc::weak_count(&alice)
    );
    println!(
        "  Bob 强引用计数 = {}, 弱引用计数 = {}",
        Rc::strong_count(&bob),
        Rc::weak_count(&bob)
    );
    // 添加 Alice 和 Charlie 为朋友
    User::add_friend(&alice, &charlie);
    println!("[添加好友] Alice 和 Charlie");
    println!(
        "  Alice 强引用计数 = {}, 弱引用计数 = {}",
        Rc::strong_count(&alice),
        Rc::weak_count(&alice)
    );
    println!(
        "  Charlie 强引用计数 = {}, 弱引用计数 = {}",
        Rc::strong_count(&charlie),
        Rc::weak_count(&charlie)
    );
    // 添加 Bob 和 Charlie 为朋友
    User::add_friend(&bob, &charlie);
    println!("[添加好友] Bob 和 Charlie");
    println!(
        "  Bob 强引用计数 = {}, 弱引用计数 = {}",
        Rc::strong_count(&bob),
        Rc::weak_count(&bob)
    );
    println!(
        "  Charlie 强引用计数 = {}, 弱引用计数 = {}",
        Rc::strong_count(&charlie),
        Rc::weak_count(&charlie)
    );
    println!("--------------------------------");

    // 创建临时用户 Dave 来演示弱引用
    {
        let dave = User::new("Dave");
        println!(
            "[创建临时用户] Dave, 初始强引用计数 = {}",
            Rc::strong_count(&dave)
        );
        // 添加 Alice 和 Dave 为朋友
        // 注意：这里只单向添加，避免循环引用
        alice
            .borrow_mut()
            .friends
            .borrow_mut()
            .push(Rc::downgrade(&dave));
        println!("[添加好友] Alice 和 Dave（弱引用）");
        println!(
            "  Alice 强引用计数 = {}, 弱引用计数 = {}",
            Rc::strong_count(&alice),
            Rc::weak_count(&alice)
        );
        println!(
            "  Dave 强引用计数 = {}, 弱引用计数 = {}",
            Rc::strong_count(&dave),
            Rc::weak_count(&dave)
        );
        // 展示 Alice 的朋友（包括 Dave）
        println!("[展示 Alice 的完整朋友列表]");
        alice.borrow().show_friends();
        // 作用域结束，Dave 将被销毁
        println!("[临时用户 Dave 离开作用域]");
    }
    println!("--------------------------------");

    // 展示最终朋友关系
    // 展示 Alice 的朋友列表（Dave 已消失）
    println!("[Alice 的最终朋友列表]");
    alice.borrow().show_friends();
    // 展示 Bob 的朋友列表
    println!("[Bob 的朋友列表]");
    bob.borrow().show_friends();
    // 展示 Charlie 的朋友列表
    println!("[Charlie 的朋友列表]");
    charlie.borrow().show_friends();
    println!("--------------------------------");

    // 最终引用计数状态
    println!("程序结束前引用计数状态:");
    println!(
        "Alice: 强引用计数 = {}, 弱引用计数 = {}",
        Rc::strong_count(&alice),
        Rc::weak_count(&alice)
    );
    println!(
        "Bob: 强引用计数 = {}, 弱引用计数 = {}",
        Rc::strong_count(&bob),
        Rc::weak_count(&bob)
    );
    println!(
        "Charlie: 强引用计数 = {}, 弱引用计数 = {}",
        Rc::strong_count(&charlie),
        Rc::weak_count(&charlie)
    );
}
