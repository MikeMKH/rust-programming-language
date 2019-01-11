use std::fmt;
use std::ops::{Add, Mul};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_display_an_ip4() {
        let home = IpAddr::V4(127, 0, 0, 1);
        assert_eq!(format!("{}", home), "127.0.0.1");
    }

    #[test]
    fn it_can_display_an_ip6() {
        let loopback = IpAddr::V6(String::from("::1"));
        assert_eq!(format!("{}", loopback), "::1");
    }

    #[test]
    fn it_can_add_some_some() {
        assert_eq!(add_option(Some(2), Some(3)), Some(5));
    }

    #[test]
    fn it_can_add_none_some() {
        assert_eq!(add_option(None, Some(3)), None);
    }

    #[test]
    fn it_can_add_some_none() {
        assert_eq!(add_option(Some(2), None), None);
    }

    #[test]
    fn it_can_add_none_none() {
        assert_eq!(add_option(None, None), None);
    }

    #[test]
    fn it_fizz_for_Fizz() {
        assert_eq!(format!("{}", FizzBuzz::Fizz(3)), "fizz");
    }

    #[test]
    fn it_buzz_for_Buzz() {
        assert_eq!(format!("{}", FizzBuzz::Buzz(5)), "buzz");
    }

    #[test]
    fn it_fizzbuzz_for_Both() {
        assert_eq!(format!("{}", FizzBuzz::Both(0)), "fizzbuzz");
    }

    #[test]
    fn it_gives_value_for_Other() {
        assert_eq!(format!("{}", FizzBuzz::Other(4)), "4");
    }

    #[test]
    fn it_fizz_for_3() {
        assert_eq!(format!("{}", fizz_buzz(3)), "fizz");
    }

    #[test]
    fn it_fizz_for_33() {
        assert_eq!(format!("{}", fizz_buzz(33)), "fizz");
    }

    #[test]
    fn it_buzz_for_5() {
        assert_eq!(format!("{}", fizz_buzz(5)), "buzz");
    }

    #[test]
    fn it_buzz_for_55() {
        assert_eq!(format!("{}", fizz_buzz(55)), "buzz");
    }

    #[test]
    fn it_fizzbuzz_for_15() {
        assert_eq!(format!("{}", fizz_buzz(15)), "fizzbuzz");
    }

    #[test]
    fn it_fizzbuzz_for_1515() {
        assert_eq!(format!("{}", fizz_buzz(1515)), "fizzbuzz");
    }

    #[test]
    fn it_4_for_4() {
        assert_eq!(format!("{}", fizz_buzz(4)), "4");
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl std::fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            IpAddr::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(ref addr) => addr.clone(),
        };
        write!(f, "{}", printable)
    }
}

fn add_option(left: Option<u32>, right: Option<u32>) -> Option<u32> {
    match left {
        Some(n) => match right {
            Some(m) => Some(n + m),
            None => None,
        },
        None => None,
    }
}

enum FizzBuzz {
    Fizz(u32),
    Buzz(u32),
    Both(u32),
    Other(u32),
}

impl std::fmt::Display for FizzBuzz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            FizzBuzz::Fizz(_) => String::from("fizz"),
            FizzBuzz::Buzz(_) => String::from("buzz"),
            FizzBuzz::Both(_) => String::from("fizzbuzz"),
            FizzBuzz::Other(value) => format!("{}", value),
        };
        write!(f, "{}", printable)
    }
}

fn fizz_buzz(value: u32) -> FizzBuzz {
    match (value % 3 == 0, value % 5 == 0) {
        (true, true) => FizzBuzz::Both(value),
        (true, false) => FizzBuzz::Fizz(value),
        (false, true) => FizzBuzz::Buzz(value),
        (false, false) => FizzBuzz::Other(value),
    }
}
