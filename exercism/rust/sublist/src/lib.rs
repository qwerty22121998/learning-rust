#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match (a.len(), b.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (m, n) => {
            let revert = m > n;
            let eq = m == n;
            let (sup, sub) = if revert {
                (a, b)
            } else {
                (b, a)
            };
            let mut contain = false;
            for window in sup.windows(sub.len()) {
                if window == sub {
                    contain = true;
                    break;
                }
            }
            match (contain, eq, revert) {
                (true, true, _) => Comparison::Equal,
                (true, false, true) => Comparison::Superlist,
                (true, false, false) => Comparison::Sublist,
                _ => Comparison::Unequal
            }
        }
    }
}
