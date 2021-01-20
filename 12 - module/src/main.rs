mod float; // import float module
mod word; // import word module from sub dir

fn main() {
    println!("{}", sum(1, 2));
    // mod calculator
    println!("{}", caculator::mul(10, 20));
    println!("{}", caculator::equation::simple(5, 9));
    println!("{}", caculator::abs_diff(1, 8));

    // mod float
    println!("{}", float::sum(10.2, 5.3));

    // mod word
    println!("{}", word::hello_word());
    println!("{}", word::vietnamese::flirt());
    println!("{}", word::vietnamese::capital());
    println!("{}", word::capital());
}

fn sum(a: i64, b: i64) -> i64 {
    a + b
}

#[test]
fn test_sum() {
    assert_eq!(3i64, sum(1i64, 2i64))
}

// module
mod caculator {
    pub fn mul(a: i64, b: i64) -> i64 {
        a * b
    }

    //nested module
    pub mod equation {
        // x + a = y
        pub fn simple(a: i64, y: i64) -> i64 {
            super::diff(y, a) // call to parent private method
        }
    }

    fn diff(a: i64, b: i64) -> i64 {
        a - b
    }

    pub fn abs_diff(a: i64, b: i64) -> i64 {
        let d = self::diff(a, b); // self is optional
        if d < 0 {
            return -d;
        }
        d
    }
}

#[cfg(test)] // only compile when running tests
mod tests {

    use super::caculator; // import root module

    #[test]
    fn test_abs_diff() {
        assert_eq!(5, caculator::abs_diff(7, 12))
    }
}
