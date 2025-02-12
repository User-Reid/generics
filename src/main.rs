#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

fn main() {
    let gold_chest: TreasureChest<&str> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };

    let silver_chest: TreasureChest<i32> = TreasureChest {
        captain: String::from("Blackbeard"),
        treasure: 8999,
    };

    let special_chest: TreasureChest<[&str; 3]> = TreasureChest {
        captain: String::from("Captain Underpants"),
        treasure: ["hi", "Hello", "Howdy"]
    };

    println!("{:?}", gold_chest);
    println!("{:?}", silver_chest);
    println!("{:?}", special_chest);
}
