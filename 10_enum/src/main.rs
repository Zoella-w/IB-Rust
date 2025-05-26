use std::{collections::HashMap, fs::File, net::TcpStream, num::ParseIntError};

fn main() {
    // #[derive(Debug)]
    // enum Pets {
    //     Cat(String),
    //     Dog { name: String, age: usize },
    //     Bird, // unit type
    // }
    // let cat = Pets::Cat;
    // let dog = Pets::Dog {
    //     name: "Alan".to_string(),
    //     age: 18,
    // };
    // // println!("dog id {dog}"); // `Pets` doesn't implement `std::fmt::Display`
    // println!("dog is {:?}", dog); // `Pets` doesn't implement `Debug`

    // // 方法
    // impl Pets {
    //     // 入参是 mut 或 &mut
    //     fn speak(&self) {
    //         println!("hi");
    //     }
    // }
    // dog.speak();

    // // 关联函数
    // impl Pets {
    //     fn log(name: String) {
    //         println!("name is {name}");
    //     }
    // }
    // // dog.log(); // error: no method named `log` found for enum `Pets`
    // Pets::log("Alan".to_string());

    // #[derive(PartialEq)]
    // enum Pets {
    //     Cat,
    //     Dog,
    // }
    // let cat = Pets::Cat;
    // let dog = Pets::Dog;
    // if cat == dog {
    //     // binary operation `==` cannot be applied to type `Pets`
    //     println!("cat == dog");
    // }
    // match cat {
    //     Pets::Cat => {
    //         println!("is cat");
    //     }
    //     Pets::Dog => {
    //         println!("is dog");
    //     }
    // }
    // if let cat = Pets::Cat {
    //     println!("is cat");
    // }
    // let num = 1;
    // match num {
    //     1 => {}
    //     2 => {}
    //     _ => {}
    // }

    // let num = Some(1);
    // let none: Option<usize> = None; // Option<T>
    // match num {
    //     Some(val) => {}
    //     None => {}
    // }

    // let map: HashMap<&str, usize> = HashMap::new();
    // let a = map.get("a");
    // match a {
    //     Some(val) => {},
    //     None => {}
    // }

    // let vec = vec![1, 2, 3];
    // let last_one = vec.iter().last();
    // match last_one {
    //     Some(val) => {}
    //     None => {}
    // }

    // let len: Result<usize, ParseIntError> = "24".parse();
    // match len {
    //     Ok(_) => {}
    //     Err(_) => {}
    // }
    // let _file = File::open("a.txt");
    // let _tcp_connection = TcpStream::connect("127.0.0.1:3000");

    // let opt: Option<i32> = Some(42);
    // let result: Result<i32, &str> = opt.ok_or("error");
    // assert_eq!(result, Ok(42));
    // let none: Option<i32> = None;
    // let result: Result<i32, &str> = none.ok_or("error");
    // assert_eq!(result, Err("error"));

    // let res: Result<i32, &str> = Ok(42);
    // let opt: Option<i32> = res.ok();
    // assert_eq!(opt, Some(42));
    // let res: Result<i32, &str> = Err("error");
    // let opt: Option<i32> = res.ok();
    // assert_eq!(opt, None);

    // let opt: Option<i32> = Some(1);
    // let a1 = opt.map(|num| num > 0); // Some(true)
    // assert!(a1.unwrap()); // true
    // let b1 = opt.and_then(|val| Some(val + 1)); // Some(2)
    // assert_eq!(b1, Some(2));
    // let c1 = opt.or_else(|| Some(2)); // Some(1)；如果不是 Some，返回 Some(2)
    // assert_eq!(c1, Some(1));
    // let ret: Result<i32, &str> = Ok(1);
    // let a2 = ret.map(|val| val > 0); // Ok(true)
    // assert!(a2.unwrap()); // true
    // let b2 = ret.and_then(|val| Ok((val + 1))); // Ok(2)
    // assert_eq!(b2, Ok(2));
    // let c2 = ret.or_else(|str| Err(3)); // Ok(1)；如果不是 Ok，返回 Err(3)
    // assert_eq!(c2, Ok(1));
    // println!("done");

    // 课后习题
    enum MyEnum {
        A(u8, u8), // 2
        B,
        C {},
    }
    // 标签1，内存2
    // 1 + 2 = 3
    println!("size of MyEnum: {}", size_of::<MyEnum>());

    enum EnumA {
        A = 255,
    }
    // 当枚举只有一个变体时，Rust 编译器会将其优化为 ​​零大小类型（ZST）​​，即不占用任何内存
    // 即使显式指定判别值，判别值仅在 ​​编译时存在​​，不会在运行时存储
    // 标签0，内存0
    println!("size of EnumA: {}", size_of::<EnumA>());

    enum EnumB {
        A = 255,
        B, // 256 -> 2
    }
    // 当定义枚举时，如果某个变体​​没有显式指定判别值（discriminant）​​
    // Rust 会默认将其判别值设为 ​​前一个变体的判别值加 1​​
    // 判别值的整数值直接用于标识变体，无需额外标签
    // 标签0，内存2
    // 0 + 2 = 2
    println!("size of EnumB: {}", size_of::<EnumB>());
}

// fn main() -> Result<(), ()> {
//     let num: Result<usize, ()> = Ok(1);
//     match num {
//         Ok(val) => {}
//         Err(_) => {}
//     }
//     // Err(())
//     Ok(())
// }
