#![allow(unused)]

use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    map.insert(address, amount);
    map
}

fn main() {
    // Create a new HashMap
    let mut scores: HashMap<String, u32> = HashMap::new();

    // Insert key-value pairs
    scores.insert("red".to_string(), 100);
    scores.insert("blue".to_string(), 200);

    // Print the entire HashMap
    println!("{:#?}", scores);

    // Get value for "red"
    let score: Option<&u32> = scores.get("red");
    println!("Red score: {:?}", score);

    // Get value for "green" (doesn't exist, will be None)
    let score: Option<&u32> = scores.get("green");
    println!("Green score: {:?}", score);

    // Update
     let score: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *score += 100; // Increase the score by 100

    // Verify the update
    let score: Option<&u32> = scores.get("black");
    println!("Black score: {:?}", score);
}
