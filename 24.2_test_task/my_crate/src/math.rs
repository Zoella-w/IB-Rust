/// 两个数的加法
///
/// # 示例
///
/// ```
/// use my_crate::math::add;
///
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 两个数的乘法
///
/// # 示例
///
/// ```
/// use my_crate::math::multiply;
///
/// let result = multiply(2, 3);
/// assert_eq!(result, 6);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// 斐波那契数列计算
///
/// # 示例
///
/// ```
/// use my_crate::math::fibonacci;
///
/// assert_eq!(fibonacci(0), 0);
/// assert_eq!(fibonacci(1), 1);
/// assert_eq!(fibonacci(10), 55);
/// ```
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// 单元测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(0, 100), 0);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }
}
