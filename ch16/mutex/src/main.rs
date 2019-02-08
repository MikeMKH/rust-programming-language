use std::sync::Mutex;

fn main() {
    let m = Mutex::new(42);
    println!("before {:?}", m);
    
    {
        let mut num = m.lock().unwrap();
        *num = 8;
    }
    
    println!("after  {:?}", m);
}
