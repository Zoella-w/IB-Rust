use std::ops::Add;
use std::str;

// fn main() {
//     // let a = String::from("hello");
//     // // {
//     // //     let b = a;
//     // //     println!("b: {b}");
//     // // }
//     // // println!("a: {a}"); // error: value moved here
//     // {
//     //     let b = &a;
//     //     let c = &a[1..2]; // e
//     //     println!("b: {b}");
//     //     println!("c: {c}");
//     // }
//     // println!("a: {a}");

//     let s = String::from("Hello, world!");
//     println!("{s}");
// }

// fn main() {
//     // // [u8] | Vec<u8> -> &str
//     // // let b = [104, 101, 108, 108, 111, 32, 228, 184, 173, 229, 155, 189];
//     // // 实现了从 Vec<u8> 到 [u8] 的隐式类型转换
//     // let b = vec![104, 101, 108, 108, 111, 32, 228, 184, 173, 229, 155, 189];
//     // let s = str::from_utf8(&b).unwrap();
//     // println!("{}", s);

//     // // 字符串字面量 -> [u8]
//     // let s = "hello 中国";
//     // let b = s.as_bytes();
//     // // println!("{:?}", b);
//     // dbg!(b);

//     // // string -> &str
//     // let s = String::from("hello, world");
//     // say_hello(&s);
//     // say_hello(&s[1..3]);
//     // say_hello(s.as_str());
//     // // let s = String::from("hello 中国");
//     // // _say_hello(&s[1..7]); // error: 索引的字节没落在字符的边界

//     // &str -> String
//     let s = String::from("hello 世界");
//     println!("{}", s);
// }

// fn _say_hello(s: &str) {
//     println!("{s}");
// }

fn main() {
    // let mut s = String::from("Hello ");
    // s.push_str("rust");
    // println!("追加字符串 push_str() -> {}", s);
    // s.push('!');
    // println!("追加字符 push() -> {}", s);

    // let mut s = String::from("Hello rust!中文");
    // s.insert(5, ',');
    // println!("插入字符 insert() -> {}", s);
    // // s.insert(13, ','); // error
    // s.insert_str(6, " I like");
    // println!("插入字符串 insert_str() -> {}", s);

    // let s = String::from("I like rust. Learning rust is my favorite!");
    // let new_string_replace = s.replace("rust", "RUST");
    // println!("{new_string_replace}");
    // // dbg!(new_string_replace);
    // let s = "I like rust. Learning rust is my favorite!";
    // let new_string_replacen = s.replacen("rust", "RUST", 1);
    // println!("{new_string_replacen}");
    // let mut s = String::from("hello rust 中国");
    // s.replace_range(6..7, "R");
    // // s.replace_range(7..13, "R"); // 不在“中”的边界
    // println!("{s}");

    // let mut s = String::from("中文!");
    // let p1 = s.pop();
    // let p2 = s.pop();
    // dbg!(p1);
    // dbg!(p2);
    // dbg!(&s);
    // let p3 = s.pop();
    // let p4 = s.pop();
    // dbg!(p3);
    // dbg!(p4);
    // dbg!(&s);

    // let mut s = String::from("测试remove方法");
    // println!(
    //     "string_remove 占 {} 个字节",
    //     std::mem::size_of_val(s.as_str())
    // );
    // // 删除第一个汉字
    // s.remove(0);
    // // 下面代码会发生错误
    // // string_remove.remove(1);
    // // 直接删除第二个汉字
    // // s.remove(3);
    // dbg!(s); // 试remove方法

    // let mut s = String::from("测试truncate");
    // s.truncate(3); // 测
    // dbg!(s);

    // let mut s = String::from("string clear");
    // s.clear();
    // dbg!(s); // ""

    // let s = String::from("hello ");
    // let string_rust = String::from("rust");
    // let result = s + &string_rust; // &string_rust 会自动解引用转为 &str
    // let mut result = result + "!";
    // result += "!!!";
    // println!("连接字符串 + -> {}", result);

    // let s = String::from("hello ");
    // let string_rust = String::from("rust");
    // let result = s.add(&string_rust);
    // println!("连接字符串 + -> {}", result);

    // let s = String::from("hello ");
    // let string_rust = String::from("rust");
    // let result = s + &string_rust; // &string_rust会自动解引用为&str
    // // println!("{}", s); // error：value borrowed here after move
    // let mut result = result + "!";
    // result += "!!!";
    // println!("连接字符串 + -> {}", result);

    // let s1 = "hello";
    // let s2 = String::from("rust");
    // let s = format!("{} {}!", s1, s2);
    // println!("{}", s); // hello rust!

    // // 通过 \ + 字符的十六进制表示，转义输出一个字符
    // let byte_escape = "I'm writing \x52\x75\x73\x74!";
    // println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape); // What are you doing? (\x3F means ?) I'm writing Rust!

    //    // \u 可以输出一个 unicode 字符
    //     let unicode_codepoint = "\u{211D}";
    //     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    //     println!(
    //         "Unicode character {} (U+211D) is called {}",
    //         unicode_codepoint, character_name
    //     ); // Unicode character ℝ (U+211D) is called "DOUBLE-STRUCK CAPITAL R"

    //     // 换行了也会保持之前的字符串格式
    //     // 使用\忽略换行符
    //     let long_string = "String literals
    //                     can span multiple lines.
    //                     The linebreak and indentation here ->\
    //                     <- can be escaped too!";
    //     println!("{}", long_string);

    println!("{}", "hello \\x52\\x75\\x73\\x74");
    // 字符串之前加 r
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
