fn main() {
    // init
    let mut a: Vec<char> = Vec::new();
    a.push('a');
    a.push('b');
    println!("{:?}", a);

    let mut a = vec![];
    a.push(1);
    a.push(2);
    println!("{:?}", a);

    let a = vec![1f64, 2.2, 4.4];
    println!("{:?}, len = {}, cap = {}", a, a.len(), a.capacity());

    let a = vec![1; 10];
    println!("{:?}", a);

    let mut a: Vec<i64> = Vec::with_capacity(5);
    a.push(1);
    a.push(2);
    println!("{:?}, len = {}, cap = {}", a, a.len(), a.capacity());

    // mutate
    let mut a: Vec<i64> = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.pop();
    println!("{:?}", a);

    let mut a = vec![1, 2, 4];
    a[1] = 5;
    println!("{:?}", a);

    let mut a: Vec<i64> = Vec::new();

    for i in 0..=10 {
        a.push(i - 5);
    }
    println!("{:?}", a);

    for e in &a {
        println!("e = {}", e);
    }
    for e in &mut a {
        println!("mut e = {}", e);
    }

    for e in a {
        println!("take ownership e = {}", e);
    }
}
