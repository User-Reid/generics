fn identity<T>(value: T) -> T {
    value
}

#[derive(Debug)]
struct Delisandwhich {}

fn main() {
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<i8>(5));
    println!("{}", identity(true));
    println!("{}", identity(String::from("Tacobell")));
    println!("{}", identity(2.234));
    println!("{:?}", identity(Delisandwhich {}));
}
