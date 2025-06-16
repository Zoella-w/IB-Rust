//! 命令行接口工具
//!
//! 运行命令: cargo run --bin cli

use my_crate::math;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    Add { a: i32, b: i32 },
    Multiply { a: i32, b: i32 },
    Fib { n: u64 },
}

fn main() {
    let cmd = Command::from_args();

    match cmd {
        Command::Add { a, b } => println!("{} + {} = {}", a, b, math::add(a, b)),
        Command::Multiply { a, b } => println!("{} * {} = {}", a, b, math::multiply(a, b)),
        Command::Fib { n } => println!("fib({}) = {}", n, math::fibonacci(n)),
    }
}
