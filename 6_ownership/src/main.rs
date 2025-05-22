// fn main() {
//     let x = 5;
//     let y = x;
//     println!("The value of x is {x}");

//     let x1 = String::from("hello");
//     let y1 = x1;
//     // println!("The value of x1 is {x1}"); // error
//     println!("The value of y1 is {y1}");
// }

// /**
//  * 课后习题
//  */
// fn take_ownership(s: String) -> String {
//     s
// }
// fn main() {
//     let s1 = String::from("Hello");
//     let s2 = take_ownership(s1);
//     // 如下代码不能修改
//     println!("{}", s1);
//     println!("{}", s2);
// }

/**
 * 答案一
 */
fn take_ownership(s: &String) -> &String {
    s
}
fn main() {
    let s1 = String::from("Hello");
    let s2 = take_ownership(&s1);
    // 如下代码不能修改
    println!("{}", s1);
    println!("{}", s2);
}

// /**
//  * 答案二
//  */
// fn take_ownership(s: String) -> String {
//     s
// }
// fn main() {
//     let s1 = String::from("Hello");
//     let s2 = take_ownership(s1.clone());
//     // 如下代码不能修改
//     println!("{}", s1);
//     println!("{}", s2);
// }
