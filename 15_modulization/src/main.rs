use a::b::c::log_c;
// alias
use a::{b::log as log_b, log as log_a};

fn main() {
    crate::a::echo(); // 绝对路径访问
    a::echo(); // 相对路径访问
    a::b::log();
    // a::b::b::log_c();
    log_c();
    log_a();
    log_b();

    let a: u8 = 1;
    add(a as usize, 1);
    fn add(a: usize, b: usize) -> usize {
        a + b
    }
}

// module
mod a {
    const num: usize = 1;
    // public
    pub fn echo() {
        //
    }
    pub fn log() {
        //
    }
    // private(default)
    fn _echo1() {
        //
    }

    pub mod b {
        use super::_echo1;

        pub fn log() {
            //
        }
        fn _echo_b() {
            _echo1(); // 子模块可以调用父模块
            self::_echo1();
        }

        pub mod c {
            pub fn log_c() {
                //
            }
        }
    }
}

// public
