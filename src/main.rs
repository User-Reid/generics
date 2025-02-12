fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn main() {
    make_tuple(String::from("Tacobell"), String::from("Banana"));
}
