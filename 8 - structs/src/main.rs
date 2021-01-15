struct User {
    name: String,
    age: u8,
}

fn c_like() {
    let me = User {
        name: String::from("vuhk"),
        age: 22,
    };

    println!("me = User({}, {})", me.name, me.age);

    let mut me = User {
        name: String::from("vuhk"),
        age: 22,
    };
    me.age = 23;
    println!("mut me = User({}, {})", me.name, me.age);

    let old_me = User { age: 60, ..me };
    println!("old me = User({}, {})", old_me.name, old_me.age);

    let User { name, age } = old_me;
    println!("name = {}, age = {}", name, age)
}

struct RGB(u8, u8, u8);

struct Status(bool);

fn tuple() {
    let black = RGB(0, 0, 0);

    println!("Black = RGB({}, {}, {})", black.0, black.1, black.2);

    let active = Status(true);

    println!("active is {}", active.0)
}

struct Empty;

fn unit() {
    let x = Empty;
}

fn main() {
    c_like();
    tuple();
    unit();
}
