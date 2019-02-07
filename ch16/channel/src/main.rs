use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("We are the Borg."),
            String::from("Lower your shields and surrender your ships."),
            String::from(
                "We will add your biological and technological distinctiveness to our own.",
            ),
            String::from("Your culture will adapt to service us."),
            String::from("Resistance is futile."),
        ];

        for m in messages {
            tx.send(m).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}
