use std::thread;
use std::time::Duration;

fn main() {
    let message = String::from("the secret word is move");
    
    let handle = thread::spawn(move || {
        println!("thread: message={}", message);
    });
    
    handle.join().unwrap();
}
