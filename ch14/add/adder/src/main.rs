extern crate succ;
extern crate succ2;
extern crate rand;

use succ::*;
use succ2::*;

fn main() {
    let value = rand::random::<u32>();
    println!("Hello, world! {} plus one is {} plus two is {}", value, succ(value), succ2(value));
}
