use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let delay = 10;
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
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
            tx1.send(m).unwrap();
            thread::sleep(Duration::from_millis(delay));
        }
    });

    thread::spawn(move || {
        for i in 1..9 {
            tx.send(i.to_string()).unwrap();
            thread::sleep(Duration::from_millis(delay));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}
