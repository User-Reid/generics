#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    fn clean_treasure_chest(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

fn main() {
    let gold_chest: TreasureChest<&str> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    println!("{gold_chest:?}");

    let mut silver_chest: TreasureChest<String> = TreasureChest {
        captain: String::from("Mac"),
        treasure: String::from("                Silver          "),
    };
    println!("{silver_chest:?}");
    silver_chest.clean_treasure_chest();
    println!("{silver_chest:?}");

    let special_chest: TreasureChest<[&str; 3]> = TreasureChest {
        captain: String::from("Hook"),
        treasure: ["Hello", "Hi", "Howdy"],
    };
    println!("{}", special_chest.amount_of_treasure());
    println!("{special_chest:?}");
}
