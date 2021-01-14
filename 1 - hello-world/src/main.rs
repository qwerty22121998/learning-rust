fn main() {
    println!("Hello, world!");
    println!("{0}, {1}!", "Hello", "world");
    println!("{who} {what}", who="Your mom", what="so fat");

    let s = format!("{a} {b}",a="Your mom so fat,", b ="I took a picture of her last Christmas and it's still printing");

    println!("{}", s);

}
