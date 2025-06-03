// use std::fs::File;
// use std::io::ErrorKind;
// use std::io::Read;
fn main() {
    //     // let mut s = String::from("A");
    //     // let p1 = s.pop();
    //     // dbg!(p1); // Some('A')
    //     // let p2 = s.pop();
    //     // dbg!(p2); // None

    //     // let five = Some(5);
    //     // let six = _plus_one(five);
    //     // let none = _plus_one(None);
    //     // println!("{:?}", six); // Some(6)
    //     // println!("{:?}", none); // None

    //     // let mut s = String::from("A");
    //     // let p1 = s.pop().unwrap();
    //     // dbg!(p1); // "A"
    //     // // let p2 = s.pop().unwrap(); // panic: called `Option::unwrap()` on a `None` value

    //     // let v = [10, 40, 30];
    //     // if v.get(1).is_some() {
    //     //     // .get() 返回 Option
    //     //     println!("{}", v[1]); // 40
    //     // }

    //     // let a = 10;
    //     // let b = 0;
    //     // let result = _div(a, b).unwrap_or(0.0);
    //     // println!("Result: {}", result); // 0.0

    //     // let v = vec![1, 2, 3];
    //     // v[99];

    //     // panic!("crash and burn");

    //     // let a = 10;
    //     // let b = 0;
    //     // let result = _div1(a, b);
    //     // println!("Result: {:?}", result); // Result: Err("y is zero")

    //     // let a = -10.0;
    //     // let b = 0.0;
    //     // let result = _div2(a, b);
    //     // println!("Result: {:?}", result); // Result: Err(DivisionByZero)
    //     // let result1 = _sqrt(a);
    //     // println!("Result1: {:?}", result1); // Result1: Err(NegativeSquareRoot)

    //     // let f = File::open("hello.txt");
    //     // let f = match f {
    //     //     Ok(file) => file,
    //     //     Err(error) => {
    //     //         panic!("Problem opening the file: {:?}", error)
    //     //     }
    //     // };

    //     // let greeting_file_result = File::open("hello.txt");
    //     // // `File::open` 返回的 `Err` 成员中的值类型 `io::Error` 是一个标准库中提供的结构体
    //     // // 该结构体有一个返回 `io::ErrorKind` 值的 `kind` 方法可供调用
    //     // // `io::ErrorKind` 是一个标准库提供的枚举，其成员对应 `io` 操作可能导致的不同错误类型
    //     // let greeting_file = match greeting_file_result {
    //     //     Ok(file) => file,
    //     //     Err(error) => match error.kind() {
    //     //         ErrorKind::NotFound => match File::create("hello.txt") {
    //     //             Ok(fc) => fc,
    //     //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //     //         },
    //     //         other_error => {
    //     //             panic!("Problem opening the file: {other_error:?}");
    //     //         }
    //     //     },
    //     // };
    //     // println!("greeting_file: {:?}", greeting_file);

    //     // let f = File::open("hello.txt").unwrap();
    //     // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    //     // let line = "1\n2\n3\n4\n";
    //     // for num in line.lines() {
    //     //     // let n = num.parse::<i32>().map(|i| i * 2).unwrap();
    //     //     // println!("{n}");
    //     //     match num.parse::<i32>().map(|i| i * 2) {
    //     //         Ok(n) => println!("{n}"),
    //     //         Err(..) => {}
    //     //     }
    //     // }
}

// fn _plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn _div(a: i32, b: i32) -> Option<f64> {
//     if b != 0 {
//         Some(a as f64 / b as f64)
//     } else {
//         None
//     }
// }

// fn _div1(x: i32, y: i32) -> Result<f64, String> {
//     if y == 0 {
//         // 操作失败，与其让程序崩溃，不如把失败的原因包装在 Err 中并返回
//         Err(("y is zero").to_string())
//     } else {
//         // 此操作有效，返回包装在 Ok 中的结果
//         Ok((x / y) as f64)
//     }
// }

// #[derive(Debug)]
// pub enum MathError {
//     DivisionByZero,
//     NegativeSquareRoot,
// }

// fn _div2(x: f64, y: f64) -> Result<f64, MathError> {
//     if y == 0.0 {
//         Err(MathError::DivisionByZero)
//     } else {
//         Ok(x / y)
//     }
// }

// fn _sqrt(x: f64) -> Result<f64, MathError> {
//     if x < 0.0 {
//         Err(MathError::NegativeSquareRoot)
//     } else {
//         Ok(x.sqrt())
//     }
// }

// // Result<T, E> -> Result<T, F>
// fn _x() -> Result<(), String> {
//     let f = File::open("hello.txt").map_err(|e: std::io::Error| format!("{e}"));
//     match f {
//         Err(e) => Err(e),
//         Ok(_) => Ok(()),
//     }
// }

// fn _read_username_from_file() -> Result<String, std::io::Error> {
//     // // 打开文件，f是`Result<文件句柄,io::Error>`
//     // let f = File::open("hello.txt");
//     // let mut f = match f {
//     //     // 打开文件成功，将file句柄赋值给f
//     //     Ok(file) => file,
//     //     // 打开文件失败，将错误返回(向上传播)
//     //     Err(e) => return Err(e),
//     // };
//     let mut f = File::open("hello.txt")?;

//     // 创建动态字符串s
//     let mut s = String::new();

//     // // 从f文件句柄读取数据并写入s中
//     // match f.read_to_string(&mut s) {
//     //     // 读取成功，返回Ok封装的字符串
//     //     Ok(_) => Ok(s),
//     //     // 将错误向上传播
//     //     Err(e) => Err(e),
//     // }
//     f.read_to_string(&mut s)?;

//     Ok(s)
// }

// fn _read_username_from_file1() -> Result<String, std::io::Error> {
//     let mut s = String::new();
//     // 链式调用
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

// 课后作业
// 修复 call 函数的错误
// 当 b 为 None 时，按默认值 1
fn call(a: i32, b: i32) -> Result<f64, String> {
    let r = divide(a, b).ok_or("Division by zero".to_string())?;
    let s = sqrt(r).map_err(|e: MathError| -> String {
        match e {
            MathError::DivisionByZero => {
                return "DivisionByZero".to_string();
            }
            MathError::NegativeSquareRoot => {
                return "NegativeSquareRoot".to_string();
            }
        }
    })?;
    return Ok(s);
}

fn divide(a: i32, b: i32) -> Option<f64> {
    if b != 0 {
        Some(a as f64 / b as f64)
    } else {
        None
    }
}

#[derive(Debug)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}
