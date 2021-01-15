fn hanle<T>(_: T) {}

fn handle_d<T, U>(_: T, _: U) {}

struct Pair<T, U>(T, U);

struct Node<T, U> {
    value: Option<Pair<T, U>>,
}

fn main() {
    hanle(10);
    hanle("1234");
    handle_d("1", 'a');

    let a = Pair(10, 'a');

    println!("a = Pair({}, {})", a.0, a.1);

    let mut root = Node {
        value: Some(Pair(1, 2)),
    };
    root.value = None
}
