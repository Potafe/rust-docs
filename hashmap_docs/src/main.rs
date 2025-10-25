use std::collections::{hash_map, HashMap};

fn main() {
    println!("Hello, world!");
    let blue = String::from("yazat");
    let red = String::from("mishra");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(red, 20);

    let team_name = String::from("Blue");
    // let score = scores.get(&team_name);
    
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }
}
