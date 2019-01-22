#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, adds_two(2))
    }

    #[test]
    fn it_does_not_adds_three() {
        assert_ne!(3, adds_two(0))
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Jack");
        assert!(
            result.contains("Jack"),
            "Greeting did not contain name, value was {}",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(101);
    }

    #[test]
    #[should_panic]
    fn less_than_1() {
        Guess::new(0);
    }

    #[test]
    fn guess_just_right() {
        Guess::new(50);
    }

    #[test]
    fn this_test_will_pass() {
        assert_eq!(print_and_return_10(4), 10)
    }

    /*
    #[test]
    fn this_test_will_fail() {
        assert_eq!(print_and_return_10(4), -10)
    }
    */
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn adds_two(x: i32) -> i32 {
    x + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 1, got {}", value);
        }

        Guess { value: value }
    }
}

fn print_and_return_10(a: i32) -> i32 {
    println!("I got {}", a);
    10
}
