//! 基准测试模块
//!
//! 使用 criterion 库进行性能测量
//! black_box 防止编译器过度优化
//! 需要 nightly Rust 运行：cargo +nightly bench

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use my_crate::math;

pub fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| math::fibonacci(black_box(20))));
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
