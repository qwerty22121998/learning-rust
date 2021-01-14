fn arithmetic() {
    // + - * / %
    let a = 10;
    let b = a + 1;
    let c = a - 1;
    let d = a * 2;
    let e = a / 2;
    let f = a % 3;

    let g = 10f64 / 2f64;

    println!(
        "a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}",
        a, b, c, d, e, f, g
    )
}

fn comparision() {
    // == != < > <= >=
    let (a, b) = (1, 2);
    println!(
        "a = {a}, b = {b}
{a} == {b} = {}
{a} != {b} = {}
{a} < {b} = {}
{a} > {b} = {}
{a} <= {b} = {}
{a} >= {b} = {}
true > false = {}",
        a == b,
        a != b,
        a < b,
        a > b,
        a <= b,
        a >= b,
        true > false,
        a = a,
        b = b,
    )
}

fn logical() {
    // ! && ||
    let (a, b) = (true, false);
    println!(
        "a = {a}, b = {b}
!{a} = {}
!{b} = {}
{a} && {b} = {}
{a} || {b} = {}",
        !a,
        !b,
        a && b,
        a || b,
        a = a,
        b = b
    );

    let (a, b) = (-1, 2);
    println!(
        "a = {a}, b = {b}
!{a} = {}
!{b} = {}",
        !a, // !a = a + (-a * 2 - 1)
        !b,
        a = a,
        b = b
    );
}

fn bitwise() {
    // & | ^ << >>
    let (a, b) = (1, 2);
    println!(
        "a = {a}, b = {b}
{a} & {b} = {}
{a} | {b} = {}
{a} ^ {b} = {}
{a} << {b} = {}
{a} >> {b} = {}",
        a & b,
        a | b,
        a ^ b,
        a << b,
        a >> b,
        a = a,
        b = b
    );
}

fn compound() {
    // += -= *= /= %= &= |= ^= <<= >>=
    let mut a = 10;
    a += 1;
    a -= 1;
    a *= 2;
    a /= 2;
    a %= 3;
    println!("a = {}", a);

    a = 10;
    a &= 15;
    a |= 9;
    a ^= 4;
    a <<= 1;
    a >>= 2;
    println!("a = {}", a)
}

fn string_concat() {
    let (a, b) = ("foo", "bar");

    println!("{}", String::from(a) + b);

    println!("{}", [a, b].concat());

    let mut s = String::from(a);
    s.push_str(b);

    println!("{}", s);

    let s = format!("{}{}", a, b);

    println!("{}", s);
}

fn main() {
    arithmetic();
    comparision();
    logical();
    bitwise();
    compound();
    string_concat();
}
