use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let message = String::from("We are the Borg. Lower your shields and surrender your ships.");
        tx.send(message).unwrap();
    });
    
    let message = rx.recv().unwrap();
    println!("Received: {}", message);
}
