// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

struct User<'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // let email = String::from("someone@example.com");
    // let username = String::from("someusername123");
    let email = "someone@example.com";
    let username = "someusername123";
    let user1 = User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     active: user1.active,
    //     username: user1.username,
    //     sign_in_count: user1.sign_in_count,
    // };
    // 结构体更新语法
    let user2 = User {
        // email: String::from("another@example.com"),
        email: "another@example.com",
        ..user1 // 简写
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}, {}", black.0, origin.1); // 访问

    struct AlwaysEqual;
    let subject = AlwaysEqual;

    let user1 = User {
        // email: String::from("someone@example.com"),
        // username: String::from("someusername123"),
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
    let active = user1.active;
    println!("{}", user1.active); // true
    print_username(&user1); // someusername123

    let name = user1.username;
    println!("{}", user1.email); // someone@example.com
                                 // println!("{}", user1.username); // error
                                 // print_username(&user1); // error
}

fn print_username(user: &User) {
    println!("{}", user.username);
}
