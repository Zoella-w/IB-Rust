// fn main() {
//     // test_common_macro_declarative();

//     say_hello!();
//     say_hello! {};
//     say_hello![];
// }

// fn _test_common_macro_declarative() {
//     println!("Hello, world!"); // 常用
//     println! {"Hello, world!"};
//     println!["Hello, world!"];
//     let v = vec![1, 2, 3, 4, 5];
//     assert_eq!(1, 10);
//     panic!("Something went wrong!");
// }

// #[macro_export]
// macro_rules! say_hello {
//     () => {
//         println!("Hello, world!");
//     };
// }

// 课后习题
#[macro_export]
macro_rules! repeat {
    // 基本情况：重复0次
    ($item:expr, 0) => {
        ""
    };
    // 基本实现：重复n次
    ($item:expr, $n:expr) => {{
        // 创建足够大的字符串空间以容纳所有重复项
        let mut result = String::with_capacity($item.len() * $n);
        for _ in 0..$n {
            result.push_str($item);
        }
        result
    }};
}

macro_rules! sum {
    // 基本情况：单个元素
    ($x:expr) => { $x };
    // 基本实现：递归处理多个元素
    // $($y:expr),+ 匹配 除了第一个参数后面的所有参数
    ($x:expr, $($y:expr),+) => {{
        // $ 说明后面的 ($y),+ 是新的匹配参数
        $x + sum!($($y),+)
    }};
}

/// 查找多个值中的最大值的宏
macro_rules! max_value {
    // 基本情况：两个值比较
    ($x:expr, $y:expr) => {
        if $x > $y { $x } else { $y }
    };
    // 递归处理多个值
    ($x:expr, $($y:expr),+) => {
        max_value!($x, max_value!($($y),+))
    };
}

fn main() {
    assert_eq!(repeat!("x", 3), "xxx");
    assert_eq!(sum!(1, 2, 3, 4, 5), 15);
    assert_eq!(max_value!(1, 8, 9), 9);
}
