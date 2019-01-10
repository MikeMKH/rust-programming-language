#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_calculate_area_by_function() {
        let r = Rectangle {
            width: 2,
            height: 3,
        };
        assert_eq!(area_rec(&r), 6);
    }

    #[test]
    fn it_can_calculate_area_by_method() {
        let r = Rectangle {
            width: 2,
            height: 3,
        };
        assert_eq!(r.area(), 6);
    }

    #[test]
    fn it_can_hold_smaller() {
        let r = Rectangle {
            width: 2,
            height: 3,
        };
        let smaller = Rectangle {
            width: 1,
            height: 1,
        };
        assert!(r.can_hold(&smaller));
    }

    #[test]
    fn it_cannot_hold_larger() {
        let r = Rectangle {
            width: 2,
            height: 3,
        };
        let larger = Rectangle {
            width: 20,
            height: 30,
        };
        assert!(!r.can_hold(&larger));
    }

    #[test]
    fn it_cannot_hold_equal() {
        let r = Rectangle {
            width: 2,
            height: 3,
        };
        let equal = r.clone();
        assert!(!r.can_hold(&equal));
    }

    #[test]
    fn it_can_create_squares() {
        let square = Rectangle::square(2);
        assert_eq!(square.area(), 2 * 2);
    }
}

#[derive(Debug, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_rec(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

impl Clone for Rectangle {
    fn clone(&self) -> Rectangle {
        *self
    }
}
