#[cfg(test)]
mod tests {
    use super::*;
    use crate::List::{Cons, Empty};
    use crate::RcList::Cons as RcCons;
    use crate::RcList::Empty as RcEmpty;
    use crate::RefList::Cons as RefCons;
    use crate::RefList::Empty as RefEmpty;
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

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

    #[test]
    fn rc_cons_works() {
        let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcEmpty)))));

        assert_eq!(a, Rc::clone(&a));
    }

    #[test]
    fn counting_references() {
        let a = Rc::new(RcCons(5, Rc::new(RcEmpty)));
        assert_eq!(1, Rc::strong_count(&a));

        let b = Rc::new(RcCons(10, Rc::clone(&a)));
        assert_eq!(2, Rc::strong_count(&a));

        {
            let c = Rc::new(RcCons(15, Rc::clone(&a)));
            assert_eq!(3, Rc::strong_count(&a));

            let d = Rc::new(RcCons(20, Rc::clone(&a)));
            assert_eq!(4, Rc::strong_count(&a));
        }
        assert_eq!(2, Rc::strong_count(&a));
    }

    #[test]
    fn ref_cons_works() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(RefCons(Rc::clone(&value), Rc::new(RefEmpty)));

        assert_eq!(
            a,
            Rc::new(RefCons(Rc::new(RefCell::new(5)), Rc::new(RefEmpty)))
        );

        let b = Rc::new(RefCons(Rc::new(RefCell::new(6)), Rc::clone(&a)));
        let c = Rc::new(RefCons(Rc::new(RefCell::new(7)), Rc::clone(&a)));

        *value.borrow_mut() *= 10;

        assert_eq!(
            a,
            Rc::new(RefCons(Rc::new(RefCell::new(50)), Rc::new(RefEmpty)))
        );

        assert_eq!(
            b,
            Rc::new(RefCons(
                Rc::new(RefCell::new(6)),
                Rc::new(RefCons(Rc::new(RefCell::new(50)), Rc::new(RefEmpty)))
            ))
        );

        assert_eq!(
            c,
            Rc::new(RefCons(
                Rc::new(RefCell::new(7)),
                Rc::new(RefCons(Rc::new(RefCell::new(50)), Rc::new(RefEmpty)))
            ))
        );
    }

    #[test]
    fn it_can_build_a_tree() {
        let leaf = Rc::new(Node {
            value: "leaf",
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        {
            let branch = Rc::new(Node {
                value: "branch",
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            assert_eq!(Rc::strong_count(&leaf), 2);
            assert_eq!(Rc::strong_count(&branch), 1);

            assert_eq!(Rc::weak_count(&leaf), 0);
            assert_eq!(Rc::weak_count(&branch), 1);

            if let Some(p) = leaf.parent.borrow().upgrade() {
                assert_eq!(p.value, "branch");
            }
        }

        assert_eq!(Rc::strong_count(&leaf), 1);
        assert_eq!(Rc::weak_count(&leaf), 0);
    }

}

#[derive(Debug, PartialEq)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Empty,
}

use std::rc::Rc;

#[derive(Debug, PartialEq)]
enum RcList<T> {
    Cons(T, Rc<RcList<T>>),
    Empty,
}

use std::cell::RefCell;

#[derive(Debug, PartialEq)]
enum RefList<T> {
    Cons(Rc<RefCell<T>>, Rc<RefList<T>>),
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

use std::rc::Weak;

#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}
