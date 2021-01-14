fn test_if() {
    let a = 10;
    println!("a = {}", a);

    if a < 15 {
        println!("a < 15")
    }

    if a > 10 {
        println!("a > 10")
    } else if a < 100 {
        println!("a < 100")
    } else {
        println!("unknown")
    }

    let e10 = if a == 10 { true } else { false };

    println!("a == 10 ? {}", e10);
}

fn test_match() {
    // switch case

    let m = |a: i64| match a {
        0 => "zero",
        1 => "one",
        10 => "ten",
        _ => "unknown",
    };

    println!("{} is {}, {} is {}", 10, m(10), 100, m(100));

    let m = |(a, b)| match (a, b) {
        (10, 10) => "both",
        (10, _) => "first",
        (_, 10) => "second",
        (x, y) if x > 1 && y < 100 => "ok",
        (_, _) => "failed",
    };

    println!(
        "{:?} is {}, {:?} is {}",
        (10, 10),
        m((10, 10)),
        (2, 6),
        m((2, 6))
    );
}

fn test_loop() {
    let mut a = 0;

    loop {
        a += 1;
        if a == 2 {
            continue; // skip 2
        }
        println!("a = {}", a);
        if a == 10 {
            break;
        }
    }

    let mut a = 0;

    'l1: loop {
        a += 1;
        let mut b = 0;

        loop {
            b += 1;

            if a == 5 {
                break 'l1;
            }
            if b == 3 {
                continue;
            }

            println!("(a, b) = {:?}", (a, b));
            if b == 5 {
                break;
            }
        }
    }
}

fn test_while() {
    let mut a = 0;

    while a < 10 {
        a += 1;
        if a == 5 {
            continue;
        }

        println!("a = {}", a);
    }

    let mut a = 0;

    'l1: while a < 5 {
        a += 1;
        let mut b = 0;
        while b < 2 {
            b += 1;
            if a == 3 {
                continue 'l1;
            }

            println!("(a , b) = {:?}", (a, b));
        }
    }
}

fn test_for() {
    for i in 0..5 {
        // 0 -> 4
        println!("i = {}", i);
    }

    for j in 0..=5 {
        // 0 -> 5
        println!("j = {}", j);
    }

    for i in 0..10 {
        if i == 5 {
            continue;
        }
        if i == 8 {
            break;
        }
        println!("i = {}", i)
    }

    'l1: for i in 0..5 {
        for j in 0..3 {
            if i == 2 {
                continue 'l1;
            }
            println!("(i, j) = {:?}", (i, j))
        }
    }

    let a: [i64; 5] = [1, 2, 3, 4, 5];

    for i in 0..a.len() {
        println!("a[{}] = {}", i, a[i])
    }

    for e in a.iter() {
        println!("e = {}", e)
    }
}

fn main() {
    test_if();
    test_match();
    test_loop();
    test_while();
    test_for();
}
