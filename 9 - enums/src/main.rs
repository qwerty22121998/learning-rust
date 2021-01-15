enum Status {
    Deactive { from: i64, to: i64, reason: String },
    Active(i64),
    Unknow,
}

fn print_status(s: Status) {
    match s {
        Status::Unknow => println!("unknow"),
        Status::Deactive { from, to, reason } => {
            println!("deactive from {} to {}, reason = {}", from, to, reason)
        }
        Status::Active(from) => {
            println!("active from {}", from)
        }
    }
}

fn main() {
    let status = Status::Unknow;

    print_status(status);

    let status = Status::Deactive {
        from: 0,
        to: 2,
        reason: String::from("1234"),
    };

    print_status(status);

    let status = Status::Active(0);

    print_status(status);
}
