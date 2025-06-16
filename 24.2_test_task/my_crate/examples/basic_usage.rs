//! 基础使用示例
//!
//! 运行命令: cargo run --example basic_usage

use my_crate::math;

fn main() {
    println!("加法: 2 + 3 = {}", math::add(2, 3));
    println!("乘法: 4 * 5 = {}", math::multiply(4, 5));
    println!("斐波那契(10): {}", math::fibonacci(10));
}
