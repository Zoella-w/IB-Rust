// fn main() {
//     // let mut s = String::from("hello");
//     // // 可以有多个不可变引用
//     // let r1 = &s;
//     // let r2 = &s;
//     // println!("{r1} and {r2}");
//     // // r1 和 r2 已经销毁了
//     // // 因此可以再声明一个 s 的可变引用
//     // let r3 = &mut s;
//     // println!("{r3}");
//     // // println!("{r1} and {r2}"); // error: mutable borrow occurs here

//     // 字符串切片
//     let s = String::from("hello world");
//     let hello = &s[0..5]; // 引用 "hello"
//     let world = &s[6..11]; // 引用 "world"

//     // 数组切片
//     let arr = [1, 2, 3, 4, 5];
//     let slice = &arr[1..3]; // 引用 [2, 3]
// }

/**
 * 课后作业
 */
fn test_lifetime() {
    let large = longest("a", "ab");
    println!("large one is {large}");
    // expected named lifetime parameter
    // fn longest(x: &str, y: &str) -> &str {
    // 如果函数返回引用，且该引用依赖于输入参数的引用，必须显式标注生命周期参数
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
}
fn main() {
    test_lifetime();
}
