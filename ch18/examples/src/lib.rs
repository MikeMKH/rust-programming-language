#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_let() {
        let favorite_color: Option<&str> = None;

        if let Some(color) = favorite_color {
            assert_eq!(color, "Green");
        }

        assert_eq!(None, favorite_color);
    }

    #[test]
    fn while_let() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut expected = 3;
        while let Some(top) = stack.pop() {
            assert_eq!(expected, top);
            expected -= 1;
        }

        assert_eq!(0, stack.len());
    }

    #[test]
    fn for_loop() {
        let v = vec!["Kelsey", "Jack", "Mike"];
        let mut visited = Vec::new();

        let mut expected = 0;
        for (index, name) in v.iter().enumerate() {
            assert_eq!(expected, index);

            expected += 1;
            visited.push(*name);
        }

        assert_eq!(v, visited);
    }

    #[test]
    fn match_literals() {
        let value = 1;

        let answer = match value {
            0 => String::from("nothing"),
            1 => String::from("monad"),
            42 => String::from("life, the universe, and everything"),
            _ => String::from("death"),
        };

        assert_eq!("monad", answer);
    }

    #[test]
    fn match_name_varible() {
        let x = Some(8);
        let y = 42;

        let result = match x {
            Some(50) => 0,
            Some(y) => y * 100,
            _ => 10,
        };

        assert_eq!(800, result);
    }

    #[test]
    fn match_multiples() {
        let x = 2;

        let result = match x {
            0 | 1 | 2 => String::from("starting"),
            3...10 => String::from("middle"),
            _ => String::from("end"),
        };

        assert_eq!("starting", result);
    }

    #[test]
    fn match_range() {
        let x = 9;

        let result = match x {
            0...7 => 1,
            8...15 => 2,
            _ => 0,
        };

        assert_eq!(2, result);
    }

    #[test]
    fn destructing_struct() {
        let p = Point { x: 0, y: -5 };

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(-5, b);

        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(-5, y);

        let result = match p {
            Point { x, y: 0 } => x * 2,
            Point { x: 0, y } => y * 3,
            Point { x, y } => x * y,
        };
        assert_eq!(-15, result);
    }

    #[test]
    fn destructing_enum() {
        let m = Message::Move { x: 1, y: 2 };

        let result = match m {
            Message::Quit => 0,
            Message::Move { x: a, y: b } => a + b,
            Message::Write(s) => s.len() as i32,
        };

        assert_eq!(3, result);
    }

    #[test]
    fn destructing_references() {
        let points = vec![
            Point { x: 1, y: 2 },
            Point { x: 3, y: 1 },
            Point { x: 0, y: 3 },
        ];

        let result: i32 = points.iter().map(|&Point { x, y }| x + y).sum();

        assert_eq!(result, 10);
    }

    #[test]
    fn ref_match() {
        let name = Some(String::from("Jack"));

        match name {
            Some(ref dog) => assert_eq!("Jack", dog),
            None => (),
        }

        assert_eq!("Jack", name.unwrap());
    }

    #[test]
    fn ref_mut_match() {
        let mut name = Some(String::from("Jack"));

        match name {
            Some(ref mut s) => {
                assert_eq!("Jack", s);
                *s = String::from("Kelsey");
            }
            None => (),
        }

        assert_eq!("Kelsey", name.unwrap());
    }

    #[test]
    fn it_must_fizz() {
        let result = fizz_buzz(33);
        assert_eq!("Fizz", result)
    }

    #[test]
    fn it_must_buzz() {
        let result = fizz_buzz(55);
        assert_eq!("Buzz", result)
    }

    #[test]
    fn it_must_fizz_buzz() {
        let result = fizz_buzz(45);
        assert_eq!("FizzBuzz", result)
    }

    #[test]
    fn it_must_2() {
        let result = fizz_buzz(2);
        assert_eq!("2", result)
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn fizz_buzz(n: i32) -> String {
    let mut result = match n {
        x if x % 15 == 0 => String::from("FizzBuzz"),
        x if x % 3 == 0 => String::from("Fizz"),
        x if x % 5 == 0 => String::from("Buzz"),
        x => String::from(x.to_string()),
    };

    result.to_string()
}
