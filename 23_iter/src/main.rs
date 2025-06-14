// fn main() {
//     _iter_study();

//     let mut mut_counter = Counter::new();
//     while let Some(count) = mut_counter.next() {
//         println!("count: {count}");
//     }
// }

// fn _iter_study() {
//     // let v1 = vec![1, 2, 3];
//     // let v1_iter = v1.iter();
//     // // rust 的迭代器是懒惰的，除非调用消费迭代器的方法，否则本身没有效果
//     // for val in v1_iter {
//     //     println!("got: {val}");
//     // }

//     // let numbers = vec![1, 2, 3, 4, 5];
//     // let mut iter = numbers.iter(); // 必须使用 mut
//     // while let Some(num) = iter.next() {
//     //     println!("{}", num);
//     // }

//     // let v1 = vec![1, 2, 3];
//     // let v1_iter = v1.iter();
//     // let total: i32 = v1_iter.sum();
//     // println!("total: {total}");

//     // let numbers = vec![1, 2, 3, 4, 5];
//     // let squares: Vec<_> = numbers.iter().map(|x| x * x).collect();
//     // println!("{:?}", squares);

//     let numbers = vec![1, 2, 3, 4, 5];
//     let even_numbers: Vec<_> = numbers.iter().filter(|&x| x % 2 == 0).collect();
//     println!("{:?}", even_numbers);
// }

// // struct Counter {}

// // pub trait Iterator {
// //     type Item;
// //     fn next(&mut self) -> Option<Self::Item>;
// //     // methods with default implementations elided
// // }
// // impl Iterator for Counter {
// //     type Item = u32;
// //     fn next(&mut self) -> Option<u32> {
// //         None
// //     }
// // }
// // // // conflicting implementations of trait `Iterator` for type `Counter`
// // // impl Iterator for Counter {
// // //     type Item = String;
// // //     fn next(&mut self) -> Option<String> {
// // //         None
// // //     }
// // // }

// // pub trait Iterator2<T> {
// //     fn next(&mut self) -> Option<T>;
// // }
// // impl Iterator2<u32> for Counter {
// //     fn next(&mut self) -> Option<u32> {
// //         None
// //     }
// // }
// // impl Iterator2<String> for Counter {
// //     fn next(&mut self) -> Option<String> {
// //         None
// //     }
// // }

// struct Counter {
//     count: u32,
// }

// impl Counter {
//     fn new() -> Self {
//         Counter { count: 0 }
//     }
// }

// impl Iterator for Counter {
//     type Item = u32;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.count += 1;
//         if self.count <= 5 {
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

// 课后作业
struct Fibonacci {
    cur: u64,
    next: u64,
    count: u64,
    limit: u64,
}

impl Fibonacci {
    fn new(limit: u64) -> Self {
        Fibonacci {
            cur: 1,
            next: 1,
            count: 1,
            limit: limit,
        }
    }
    fn into_vec(&mut self) -> Vec<u64> {
        let mut v = vec![];
        while let Some(count) = self.next() {
            v.push(count);
        }
        v
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur;
        self.cur = self.next;
        self.next = cur + self.next;
        if (self.count <= self.limit) {
            self.count += 1;
            Some(cur)
        } else {
            None
        }
    }
}

fn main() {
    let mut fib = Fibonacci::new(5);
    // for number in fib.take(10) {
    //     println!("{}", number);
    // }
    for number in fib.into_vec().iter() {
        println!("{}", number);
    }
}
