//! My Crate - 演示 Rust 项目规范
//!
//! 提供数学计算功能的库
//!
//! ## 功能特性
//! - 基本算术运算
//! - 斐波那契数列计算
//!
//! ## 使用示例
//!
//! ```
//! use my_crate::math;
//!
//! let sum = math::add(2, 3);
//! let product = math::multiply(4, 5);
//! ```

pub mod math;

#[cfg(test)]
mod tests {
    /// 库级的文档测试示例
    ///
    /// ```
    /// use my_crate;
    ///
    /// assert_eq!(my_crate::math::add(1, 1), 2);
    /// ```
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
