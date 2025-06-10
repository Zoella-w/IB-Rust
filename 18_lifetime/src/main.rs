fn main() {
    // let x_ref = {
    //     let x = 3;
    //     &x // borrowed value `x` does not live long enough
    // };

    let s: &'static str = "hello";
    const SOME_COORDINATE: (i32, i32) = (7, 4);
    let static_reference: &'static (i32, i32) = &SOME_COORDINATE;
}

fn _longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct _Example<'a, 'b> {
    part: &'a str,
    part2: &'b str,
}
impl<'a, 'b> _Example<'a, 'b> {
    fn echo(&self) {
        //
    }
}

enum _StringOption<'a, 'b> {
    Some(&'a str, &'b str),
    None,
}
impl<'a, 'b> _StringOption<'a, 'b> {
    fn get(&self) {
        //
    }
}

fn _first_word(s: &str) -> &str {
    &s[..1]
}
// 返回值不包含引⽤，⽆需标注
fn _add(a: &i32, b: &i32) -> i32 {
    *a + *b
}
// 只包含⼀个 引⽤ ⽣命周期⼀致
fn _identity(a: &i32) -> &i32 {
    a
}

struct Counter<'a> {
    counter: &'a mut i32,
}
impl Counter<'_> {
    fn increment(&mut self) {
        *self.counter += 1;
    }
}

fn _example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a,
{
    x
}

fn _example1<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'a: 'b,
{
    x
}

fn test_lifetime_mut() {
    fn insert_value<'a, 'b>(my_vec: &mut Vec<&'a i32>, value: &'b i32)
    where
        'b: 'a,
    {
        my_vec.push(value);
    }
    {
        let x = 1;
        let mut my_vec = vec![&x];
        let y = 2;
        insert_value(&mut my_vec, &y);
        println!("{my_vec:?}");
    }
}

// fn test_lifetime_multiple() {
//     fn insert_value<'a, 'b>(my_vec: &'a mut Vec<&'a i32>, value: &'b i32) {
//         my_vec.push(value)
//     }
//     let mut my_vec: Vec<&i32> = vec![];
//     let val1 = 1;
//     let val2 = 2;
//     let a = &mut my_vec;
//     insert_value(a, &val1);
//     println!("a is {:?} ", a);
//     let b = &mut my_vec;
//     insert_value(b, &val2);
//     println!("b is {:?}", b);
//     println!("{my_vec:?}");
// }

fn _test_lifetime_multiple() {
    fn insert_value<'a>(my_vec: &mut Vec<&'a i32>, value: &'a i32) {
        my_vec.push(value)
    }
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;
    let a = &mut my_vec;
    insert_value(a, &val1);
    println!("a is {:?} ", a);
    let b = &mut my_vec;
    insert_value(b, &val2);
    println!("b is {:?}", b);
    println!("{my_vec:?}");
}
