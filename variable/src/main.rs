fn main() {
    // Variables
    let a; // variable are immutable
    a = 5;

    let b: i8;
    b = 10;

    println!("a={}, b={}", a, b);

    let t = true;
    let f: bool = true;

    println!("t={}, f={}", t, f);

    let (x, y) = (1, 2);

    println!("(x, y)=({}, {})", x, y);

    let mut c = 10;

    println!("c before = {}", c);

    c = 100;

    println!("c after = {}", c);

    let z = { x + y };

    println!("z = {}", z);

    let z = {
        let x = 0;
        let y = 1;

        x + y
    };

    println!("z = {}", z);

    // Constant
    const N: i32 = 5;

    println!("constant N = {}", N);

    // Static
    static S: i32 = 100;

    println!("static S = {}", S);

    // Shadowning

    let x: f64 = 9.2;
    let x: i64 = x.floor() as i64;

    println!("x = {}", x);

    let s: &str = "hello";
    let s: String = s.to_uppercase();
    println!("s = {}", s);
}
