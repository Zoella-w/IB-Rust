fn main() {
    // immutable();
    // mutable();
    // fmt_const();
    // foo(5);
    // static_num();
    binding();
}

fn immutable() {
    let x = 5;
    let x1 = 10;
    let x: i32 = x1;
    println!("The value of x is: {}", x);
    // x = 10; // cannot assign twice to immutable variable
}

fn mutable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 10;
    println!("The value of x is: {}", x);
}

const NUM: i32 = 5;
const THREE_HOURS: i32 = 60 * 60 * 3;

const num: i32 = 5; // warning

fn fmt_const() {
    println!("The value of num is: {}", NUM);
    println!("The value of THREE_HOURS is: {}", THREE_HOURS);
    // THREE_HOURS = 100; // cannot assign twice to immutable variable
}

fn foo(x: i32) {
    let y = x + 10;
    println!("The value of y is: {}", y);
}

// define a static variable
// static STATIC_NUM: i32 = 5;

static mut STATIC_NUM: i32 = 5;

fn static_num() {
    unsafe {
        STATIC_NUM += 1;
        // println!("The value of STATIC_NUM is: {}", STATIC_NUM); // creating a shared reference to mutable static is discouraged
        let current = STATIC_NUM;
        println!("The value of current is: {}", current);
    }
}

fn binding() {
    // 在 main 函数中绑定生存
    let long_lived_binding: i32 = 1;

    // 代码块，比 main 函数拥有更小的作用域
    {
        // 在代码块中绑定生存
        let short_lived_binding: i32 = 2;
        println!(
            "The value of short_lived_binding is: {}",
            short_lived_binding
        );
        // short_lived_binding = 5; // cannot assign twice to immutable variable

        // shadowing 遮蔽
        let short_lived_binding: f32 = 5_f32;
        println!(
            "The value of short_lived_binding is: {}",
            short_lived_binding
        );

        println!("The value of long_lived_binding is: {}", long_lived_binding);

        // long shadowing 遮蔽
        let long_lived_binding: f32 = 5_f32;
        println!("The value of long_lived_binding is: {}", long_lived_binding);
    }

    // println!(
    //     "The value of short_lived_binding is: {}",
    //     short_lived_binding
    // ); // cannot find value `short_lived_binding` in this scope

    // 同样是 shadowing 遮蔽
    let long_lived_binding: f32 = 8_f32;
    println!("The value of long_lived_binding is: {}", long_lived_binding);
}
