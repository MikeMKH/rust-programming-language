#[cfg(test)]
mod tests {
    use crate::List::{Cons, Empty};
    use super::*;
    
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
        
        assert_eq!(x, *y);
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