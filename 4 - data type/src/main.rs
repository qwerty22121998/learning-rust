fn t_bool() {
    let x = true;
    let y: bool = false;

    println!("x = {}, y = {}", x, y)
}

fn t_char() {
    let a = 'o';
    let b: char = 'z';

    println!("a = {}, b = {}", a, b)
}

fn t_s_int() {
    let a: i8 = -100;
    let b: i16 = -20000;
    let c: i32 = 3000000;
    let d: i64 = 400000000000;
    let e: i128 = 50000000000000000;
    let f = 60000000i128;
    println!(
        "a = {}, b = {}, c = {}, d = {}, e = {}, f = {}",
        a, b, c, d, e, f
    )
}

fn t_u_int() {
    let a: u8 = 100;
    let b: u16 = 20000;
    let c: u32 = 3000000;
    let d: u64 = 400000000000;
    let e: u128 = 50000000000000000;
    println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e)
}

fn t_float() {
    let a = 5.6; // default f64
    let b: f32 = 7.8;
    let c: f64 = 100.1;
    println!("a = {}, b = {}, c ={}", a, b, c)
}

fn t_array() {
    let a = [1, 2, 3]; // array size are immutable
    let b: [i64; 5] = [1, 2, 3, 4, 5];

    let mut c: [i64; 3] = [1, 2, 3]; // only element are mutable
    c[2] = 4;

    let d = [0; 5];

    println!("a = {:?}, b = {:?}, c = {:?}, d = {:?}", a, b, c, d)
}

fn t_tuple() {
    let a = (1, 'a', "bcd");

    let mut b = ('z', "oke", 123);
    b.0 = 'a';
    b.2 = 1001;

    let (e, f, _) = b;

    let g = (('a', 1), 513, ('z', "hello"));

    println!(
        "a = {:?}, b = {:?}, e = {}, f = {}, g = {:?}",
        a, b, e, f, g
    );
}

fn t_slice() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b: &[i32] = &a;

    let c = &a[1..3];

    let d = &a[1..];

    println!("a = {:?}, b = {:?}, c = {:?}, d = {:?}", a, b, c, d);
}

fn t_str() {
    let a = "okeokeokeokeo";
    let b: &str = "ekoekoekoekoekoekoe";

    println!("a = {}, b = {}", a, b);
}

fn main() {
    t_bool();
    t_char();
    t_s_int();
    t_u_int();
    t_float();
    t_array();
    t_tuple();
    t_slice();
    t_str();
}
