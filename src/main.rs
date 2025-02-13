enum Cheesesteak<T> {
    Plain,
    Topping(T),
}

fn main() {
    let mushroom: Cheesesteak<&str> = Cheesesteak::Topping("Mushroom");
    let onions: Cheeseseak<String> = Cheesesteak::Topping(String::from("Onions"));
    let topping: String = String::from("Bacon");
    let bacon: Cheesesteak<&String> = Cheesesteak(&topping);

    let mut plain: Cheesesteak<String> = Cheesesteak::Plain;
    plain = Cheesesteak::Topping(String::from("Chicken"));
}
