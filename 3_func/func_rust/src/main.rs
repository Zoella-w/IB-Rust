fn main() {
    let a = 1;
    let b = 2;
    let c = add(a, b);

    println!("{}", c);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}
