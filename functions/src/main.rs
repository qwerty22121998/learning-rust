fn print_sum(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b)
}

fn sum(a: i32, b: i32) -> i32 {
    // with fn, param and return type are required
    a + b
}

fn main() {
    println!("Hello, world!");

    let a: i32 = 100;
    let b: i32 = 20;

    print_sum(a, b);

    println!("{} + {} = {}", a, b, sum(a, b));

    let sum_fn = sum;

    println!("sum fn: {} + {} = {}", a, b, sum_fn(a, b));

    let mul = |a: i32, b: i32| -> i32 { a * b };

    println!("{} * {} = {}", a, b, mul(a, b));
    let mul = |a: i32, b| a * b; // with lambda, param and return type are optional

    println!("{} * {} = {}", a, b, mul(a, b));

    println!(
        "{} - {} = {}",
        a,
        b,
        |a: i32, b: i32| -> i32 { a - b }(a, b)
    )
}
