fn main() {
    // print_int_show();
    // print_show();
    // nan();
    // char_show();
    // sequence();
    convert();
}

fn print_int_show() {
    let _i: i32 = 98;
    let _i: i32 = 98_222; // =98222
    let _i = 0xff; // default i32
    let _i = 0o77;
    let _i = 0b1111_0000;
    let i = b'A'; // default u8, print: 65
    println!("The value of i is {i}");
}

fn print_show() {
    // let f = 2.0; // default f64
    // let y : f32 = 3.0;

    // assert!(0.1 + 0.2 == 0.3); // assertion failed

    // assert!(((0.1_f64 + 0.2 - 0.3).abs() < 0.000_001) == true);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("0.1+0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("0.3: {:x}", (abc.2).to_bits());
    println!("0.1+0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("0.3: {:x}", (xyz.2).to_bits());
}

fn nan() {
    let v = (-1.1_f64).sqrt();
    println!("v: {}", v);
    if v.is_nan() {
        println!("v is nan");
    }
}

fn bool_show() {
    let t = true;
    let f: bool = true;
}

fn char_show() {
    let x = '中';
    let y: char = '⏰';
    println!("x: {x}, y: {y}");
    println!("{}", std::mem::size_of_val(&x));
}

fn sequence() {
    // for i in 1..=5 {
    //     println!("i: {i}");
    // }
    for i in 'a'..='z' {
        println!("i: {i}");
    }
}

fn convert() {
    let i = 5;
    let f = i as f32;
    println!("i: {i}");
    println!("f: {f}");
}
