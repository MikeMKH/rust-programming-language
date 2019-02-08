use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let factorial = Arc::new(Mutex::new(1));
    let value = 10;
    let mut handles = vec![];
    
    for i in 1..(value + 1) {
        let factorial = Arc::clone(&factorial);
        let handle = thread::spawn(move || {
            let mut val = factorial.lock().unwrap();
            
            *val *= i;
            println!("{}: {}*{}", i, *val, i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\nResult: {} factorial = {}", value, *factorial.lock().unwrap());
}
