#[cfg(test)]
mod tests {
    use super::*;
    use crate::List::{Cons, Empty};

    #[test]
    fn it_can_use_the_stack() {
        let x = 5;
        assert_eq!(x, 5);
    }

    #[test]
    fn it_can_use_the_heap() {
        let x = Box::new(5);
        assert_eq!(*x, 5);
    }

    #[test]
    fn cons_works() {
        let col = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Empty))))));
        assert_eq!(
            col,
            Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Empty))))))
        );
    }

    #[test]
    fn follow_pointer() {
        let x = 42;
        let y = &x;

        assert_eq!(x, *y);
    }

    #[test]
    fn sphere_is_like_box() {
        let x = "Hello";
        let y = Sphere(x);
        let z = Box::new(x);

        assert_eq!(x, *y);
        assert_eq!(*z, *y);
    }

    #[test]
    #[should_panic(expected = "dropping the smarts")]
    fn fake_it_until_you_make_it() {
        let x = "Hello";
        let y = IGotTheSmarts { data: x };

        assert_eq!(x, y.data);
    }

    #[test]
    #[should_panic(expected = "dropping the smarts")]
    fn drop_it_likes_it_hot() {
        let x = IGotTheSmarts { data: 42 };
        drop(x);
    }
}

#[derive(Debug, PartialEq)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Empty,
}

struct Sphere<T>(T);

impl<T> Sphere<T> {
    fn new(x: T) -> Sphere<T> {
        Sphere(x)
    }
}

use std::ops::Deref;

impl<T> Deref for Sphere<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[derive(Debug)]
struct IGotTheSmarts<T> {
    data: T,
}

impl<T> Drop for IGotTheSmarts<T> {
    fn drop(&mut self) {
        panic!("dropping the smarts")
    }
}
