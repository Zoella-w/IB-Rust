//! 集成测试 - 验证库的公共 API

use my_crate::math;

#[test]
fn test_add_integration() {
    assert_eq!(math::add(10, 20), 30);
}

#[test]
fn test_multiply_integration() {
    assert_eq!(math::multiply(5, 6), 30);
}

#[test]
fn test_fibonacci_integration() {
    assert_eq!(math::fibonacci(20), 6765);
}
