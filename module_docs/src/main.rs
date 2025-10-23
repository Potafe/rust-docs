mod garden; // tells Rust to look for a folder named garden
use crate::garden::garden::Asparagus;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
