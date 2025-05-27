use std::ffi::os_str::Display;

trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    r: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }
}

// 通用方法
fn print_area(graph: &impl Shape) {
    println!("{}", graph.area());
}

// #[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // fn area(self: &Self) -> u32 {
    //     self.width * self.height
    // }
    // fn area(self: &Rectangle) -> u32 {
    //     self.width * self.height
    // }

    fn set_width(&mut self, new_width: u32) {
        self.width = new_width;
    }

    // getter
    pub fn width(&self) -> bool {
        self.width > 0
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.height {
            let mut s = String::new();
            for _ in 0..self.width {
                s.push('#');
            }
            write!(f, "{}\n", s);
        }
        return Ok(());
    }
}

// 课后习题
impl std::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.height {
            let mut s = String::new();
            for _ in 0..self.width {
                s.push('#');
            }
            write!(f, "{}\n", s);
        }
        return Ok(());
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let mut rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    rect2.set_width(10);
    if (rect2.width()) {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect3 = Rectangle::new(30, 50);
    println!("{}", rect2.can_hold(&rect3));

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            (self.width * self.height) as f64
        }
    }

    let cir1 = Circle { r: 10.0 };
    print_area(&cir1);

    println!("rect1 is {:?}", rect1); // rect1 is Rectangle { width: 30, height: 50 }
    println!("rect1 is {:#?}", rect1);
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    // [src/rectangle/main.rs:97:16] 30 * scale = 60
    // [src/rectangle/main.rs:100:5] &rect1 = Rectangle {
    //     width: 60,
    //     height: 50,
    // }
    println!("{}", rect1);
    println!("{:?}", rect1);
}
