extern crate succ;
extern crate rand;

use succ::*;

fn main() {
    let value = rand::random::<u32>();
    println!("Hello, world! {} plus one is {}", value, succ(value));
}
