// use std::{
//     fmt::{Debug, Display},
//     ops::Add,
// };

// fn main() {
//     //     let post = Post {
//     //         title: "Rust语言简介".to_string(),
//     //         author: "Sunface".to_string(),
//     //         content: "Rust棒极了!".to_string(),
//     //     };
//     //     let weibo = Weibo {
//     //         username: "sunface".to_string(),
//     //         content: "好像微博没Tweet好用".to_string(),
//     //     };
//     //     println!("{}", post.summarize());
//     //     println!("{}", weibo.summarize());

//     //     let post1 = Post1 {
//     //         title: "Rust语言简介".to_string(),
//     //         author: "Sunface".to_string(),
//     //         content: "Rust棒极了!".to_string(),
//     //     };
//     //     println!("{}", post1.summarize());

//     // let my_int = MyInt(42);
//     // let output: String = my_int.convert(); // 调用 Converter<String>
//     // println!("output is: {}", output);
//     // let output: f32 = my_int.convert(); // 调用 Converter<f32>
//     // println!("output is: {}", output);

//     // let my_int = MyInt1(42);
//     // // 转换为 i32 默认值 (0)
//     // let num: i32 = my_int.convert();
//     // println!("i32: {}", num); // 输出: i32: 0
//     // // 转换为 String 默认值 (空字符串)
//     // let s: String = my_int.convert();
//     // println!("String: '{}'", s); // 输出: String: ''

//     // let res = add(1, 2);
//     // println!("res: {res}");
//     // let res1 = add(Point { x: 1, y: 2 }, Point { x: 3, y: 4 });
//     // println!("res1: {:?}", res1);

//     // let m1 = Millimeter(10);
//     // let m2 = Meter(1);
//     // let res = add(m1, m2);
//     // println!("res: {:?}", res);

//     // let weibo = Weibo {
//     //     username: "sunface".to_string(),
//     //     content: "好像微博没Tweet好用".to_string(),
//     // };
//     // notify(&weibo);

//     let post1 = Post::default();
//     let post2 = None.unwrap_or_default();
// }

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// #[derive(Default)]
// pub struct Post {
//     pub title: String,   // 标题
//     pub author: String,  // 作者
//     pub content: String, // 内容
// }
// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("文章{}, 作者是{}", self.title, self.author)
//     }
// }

// pub struct Weibo {
//     pub username: String,
//     pub content: String,
// }
// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{}发表了微博{}", self.username, self.content)
//     }
// }

// // // only traits defined in the current crate can be implemented for types defined outside of the crate
// // impl Display for String {}

// pub trait Summary1 {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }
// pub struct Post1 {
//     pub title: String,   // 标题
//     pub author: String,  // 作者
//     pub content: String, // 内容
// }
// impl Summary1 for Post1 {}

// // 将元组结构体中的数据，转换为类型 T
// trait Converter<T> {
//     fn convert(&self) -> T;
// }
// // 元组结构体
// struct MyInt(i32);
// impl Converter<String> for MyInt {
//     fn convert(&self) -> String {
//         // self.0 访问元组结构体的第 0 个元素
//         self.0.to_string()
//     }
// }
// impl Converter<f32> for MyInt {
//     fn convert(&self) -> f32 {
//         self.0 as f32
//     }
// }

// trait Converter1<T> {
//     fn convert(&self) -> T;
// }
// struct MyInt1(i32);
// // 约束 T: Default 表示 T 必须实现 Default trait（具有默认值）
// impl<T: Default> Converter1<T> for MyInt1 {
//     fn convert(&self) -> T {
//         // 调用 T 类型的默认值构造函数
//         T::default()
//     }
// }

// trait Converter2 {
//     type Output;
//     fn convert(&self) -> Self::Output;
// }
// impl Converter2 for MyInt {
//     type Output = String;
//     fn convert(&self) -> Self::Output {
//         self.0.to_string()
//     }
// }
// // // conflicting implementations of trait `Converter2` for type `MyInt`
// // impl Converter2 for MyInt {
// //     type Output = f32;
// //     fn convert(&self) -> Self::Output {
// //         self.0 as f32
// //     }
// // }

// // fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
// //     // 相当于 fn add<T: std::ops::Add<T, Output = T>>(a: T, b: T) -> T {
// //     a + b
// // }
// fn add<T: std::ops::Add<U, Output = T>, U>(a: T, b: U) -> T {
//     a + b
// }

// #[derive(Debug, Clone)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl Add for Point {
//     type Output = Point;
//     fn add(self, other: Self) -> Self {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// #[derive(Debug)]
// struct Millimeter(u32);
// struct Meter(u32);
// impl Add for Millimeter {
//     // 相当于 impl Add<Millimeter> for Millimeter {}
//     type Output = Millimeter;
//     fn add(self, other: Self) -> Millimeter {
//         Millimeter(self.0 + other.0 * 1000)
//     }
// }
// impl Add<Meter> for Millimeter {
//     type Output = Millimeter;
//     fn add(self, other: Meter) -> Millimeter {
//         Millimeter(self.0 + other.0 * 1000)
//     }
// }

// fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
// fn notify1(item: &(impl Summary + Display)) {}
// // fn notify<T: Summary>(item: &T) {
// //     println!("Breaking news! {}", item.summarize());
// // }
// // fn notify1<T: Summary + Display>(item: &T) {}

// fn some_func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> () {}
// fn some_func1<T, U>(t: &T, u: &U) -> ()
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

// struct Pair<T> {
//     x: T,
//     y: T,
// }
// // impl Pair<i32> {
// //     fn new(x: i32, y: i32) -> Self {
// //         Self { x, y }
// //     }
// // }
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }
// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }

// fn returns_summarizable() -> impl Summary {
//     Weibo {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//     }
// }

// fn returns_summarizable1(switch: bool) -> Box<dyn Summary> {
//     if switch {
//         Box::new(Post {
//             title: String::from("Penguins win the Stanley Cup Championship!"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         })
//     } else {
//         Box::new(Weibo {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//         })
//     }
// }

// 课后作业
use std::{fmt::Display, process::Output};

// 不要需改 Item 的定义
trait Item<T = String> {
    type Output: Display;
    fn summarize(&self) -> Self::Output;
}

// 不要需改 Apple 结构的定义
struct Apple {
    name: String,
}

impl Item for Apple {
    type Output = String;
    fn summarize(&self) -> String {
        self.name.to_string()
    }
}

// 不要需改 weibo 结构的定义
struct Weibo {
    author: String,
    content: String,
}

impl Item for Weibo {
    type Output = String;
    fn summarize(&self) -> String {
        format!("@{}:{}", self.author, self.content)
    }
}

pub struct Container {
    items: Vec<Box<dyn Item<Output = String>>>,
}

impl Container {
    pub fn iterator(&self) {
        for item in &self.items {
            println!("{}", item.summarize());
        }
    }
}

fn main() {
    let apple = Apple {
        name: "Apple".to_string(),
    };
    let w = Weibo {
        author: "weibo".to_string(),
        content: "hello".to_string(),
    };
    let container = Container {
        items: vec![Box::new(apple), Box::new(w)],
    };
    container.iterator();
}
