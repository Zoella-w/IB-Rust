// fn main() {
//     match_t();
// }

// fn match_t() {
//     // let number = 13;
//     // match number {
//     //     1 => println!("One!"),
//     //     2 => println!("Two!"),
//     //     3 => println!("Three!"),
//     //     _ => println!("Something else!"),
//     // }
//     let x = 1;
//     let y = "Hello";
//     match x {
//         1 => println!("One"),
//         2 => println!("Two"),
//         _ => println!("Other"),
//     }
//     match y {
//         "Hello" => println!("Greeting"),
//         "Goodbye" => println!("Farewell"),
//         _ => println!("Other"),
//     }

//     let x = 42;
//     let y = String::from("abc");
//     match x {
//         var => println!("The value is: {}", var),
//     }
//     match y {
//         var => println!("The value is: {}", var),
//     }
//     println!("The x is {x}");
//     // println!("The y is {y}"); // error: value borrowed here after move

//     match x {
//         _ => println!("Any value"),
//     }

//     struct Point {
//         x: i32,
//         y: i32,
//     }
//     let p = Point { x: 0, y: 7 };
//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         Point { x: 0, y } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }

//     let x = 5;
//     match x {
//         n if n % 2 == 0 => println!("Even"),
//         n => println!("Odd"),
//     }

//     match divide(4, 2) {
//         Ok(result) => println!("Result is {}", result),
//         Err(e) => println!("Error: {}", e),
//     }

//     enum Message {
//         Quit,
//         Move { x: i32, y: i32 },
//         Write(String),
//         ChangeColor(i32, i32, i32),
//     }
//     let msg = Message::ChangeColor(0, 160, 255);
//     match msg {
//         // 解构
//         Message::ChangeColor(r, g, b) => {
//             println!("Change the color to red {}, green {}, and blue {}", r, g, b)
//         }
//         _ => (),
//     }

//     let vec1 = vec![1, 2, 3];
//     let vec2 = vec![1, 2, 3];
//     for (a, b) in vec1.iter().zip(vec2) {
//         println!("{} + {} = {}", a, b, a + b);
//     }

//     let opt = Some(5);
//     if let Some(x) = opt {
//         println!("Matched {:?}", x);
//     }
//     let mut iter = vec![1, 2, 3].into_iter();
//     while let Some(x) = iter.next() {
//         println!("Matched {:?}", x);
//         // Matched 1
//         // Matched 2
//         // Matched 3
//     }

//     let x = String::from("Hello");
//     match x {
//         // 借用 x 的所有权，x 所有权没有转移
//         ref r => println!("Got a reference to a value: {:?}", r),
//     }
//     println!("x is still accessible: {}", x);
//     let mut x = String::from("Hello");
//     match x {
//         ref mut r => {
//             *r = String::from("world");
//             println!("Got a mutable reference to a value: {:?}", r)
//         }
//     }
// }

// fn divide(a: i32, b: i32) -> Result<i32, String> {
//     if b == 0 {
//         Err(String::from("Cannot divide by zero"))
//     } else {
//         Ok(a / b)
//     }
// }

// 课后习题
use serde_json::{Result, Value};
use std::collections::HashMap;

fn main() -> Result<()> {
    // 原始 JSON 数据
    // 如果字符串包含双引号，可以在开头和结尾加 #
    let json_str = r#"
    {
        "name": "Alice",
        "age": 30,
        "email": "alice@example.com",
        "address": {
            "street": "123 Main St",
            "city": "Wonderland"
        },
        "phone_numbers": ["123-456-7890", "987-654-3210"]
    }
    "#;

    // 步骤 1：解析 JSON 字符串为动态类型 Value
    // serde_json::from_str 返回 Result<Value, Error>
    let v: Value = serde_json::from_str(json_str)?;

    // 步骤 2：使用模式匹配处理 JSON 结构
    match v {
        Value::Object(obj) => {
            let mut name = String::from("Unknown");
            let mut age = 0;
            let mut address = HashMap::new();
            let mut phone_numbers = Vec::new();

            for (key, value) in obj {
                match key.as_str() {
                    "name" => {
                        if let Value::String(s) = value {
                            name = s;
                        }
                    }
                    "age" => {
                        if let Value::Number(n) = value {
                            // as_i64()​​：将 JSON 数值转换为 i64 类型，返回 Option<i64>
                            // unwrap_or(0)：解包 Option<i64>，如果值为 None，则使用默认值 0
                            age = n.as_i64().unwrap_or(0) as i32;
                        }
                    }
                    "address" => {
                        if let Value::Object(addr_obj) = value {
                            let mut street = String::from("Unknown");
                            let mut city = String::from("Unknown");

                            for (addr_key, addr_value) in addr_obj {
                                match addr_key.as_str() {
                                    "street" => {
                                        if let Value::String(s) = addr_value {
                                            street = s;
                                        }
                                    }
                                    "city" => {
                                        if let Value::String(s) = addr_value {
                                            city = s;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            // 插入 HashMap 键值对
                            address.insert("street", street);
                            address.insert("city", city);
                        }
                    }
                    "phone_numbers" => {
                        if let Value::Array(arr) = value {
                            for num in arr {
                                if let Value::String(s) = num {
                                    phone_numbers.push(s.clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            println!("Name: {}", name);
            println!("Age: {}", age);
            println!("Address: {:?}", address);
            println!("Phone Numbers: {:?}", phone_numbers);
        }
        _ => println!("Invalid JSON structure"),
    }

    Ok(())
}
