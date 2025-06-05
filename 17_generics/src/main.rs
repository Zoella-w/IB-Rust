fn main() {
    // println!("add i8: {}", add_i8(2i8, 3i8));
    // println!("add i32: {}", add_i32(20, 30));
    // println!("add f64: {}", add_f64(1.23, 1.23));

    // println!("add i8: {}", _add(2i8, 3i8));
    // println!("add i32: {}", _add(20, 30));
    // println!("add f64: {}", _add(1.23, 1.23));

    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    // // let wont_work = Point { x: 5, y: 4.0 }; // error: mismatched types
    // let integer_and_float = Point1 { x: 5, y: 4.0 };

    // let p1 = Point1 { x: 5, y: 10.4 };
    // let p2 = Point1 { x: "Hello", y: 'c' };
    // let p3 = p1.mixup(p2);
    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // let arr: [i32; 3] = [1, 2, 3];
    // display_array(arr);
    // let arr: [i32; 2] = [1, 2];
    // // display_array(arr); // error: mismatched types

    let arr: [i32; 3] = [1, 2, 3];
    display_array1(&arr);
    let arr: [i32; 2] = [1, 2];
    display_array1(&arr);

    display_array2(arr);
    display_array2(arr);
}

// fn add_i8(a: i8, b: i8) -> i8 {
//     a + b
// }
// fn add_i32(a: i32, b: i32) -> i32 {
//     a + b
// }
// fn add_f64(a: f64, b: f64) -> f64 {
//     a + b
// }
fn _add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}
struct Point1<T, U> {
    x: T,
    y: U,
}

// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

impl Point<i32> {
    fn x(&self) -> &i32 {
        &self.x
    }
}

impl<T> Point<T> {
    fn x1(&self) -> &T {
        &self.x
    }
}

impl<X1, Y1> Point1<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point1<X2, Y2>) -> Point1<X1, Y2> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

fn display_array(arr: [i32; 3]) {
    println!("{:?}", arr);
}
fn display_array1<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}
fn display_array2<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
