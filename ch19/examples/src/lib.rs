#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_pointer() {
        let mut n = 8;
        let r1 = &n as *const i32;
        let r2 = &mut n as *mut i32;

        assert_eq!(8, n);
        unsafe {
            assert_eq!(8, *r1);
            assert_eq!(8, *r2);

            *r2 = 5;

            assert_eq!(5, *r1);
            assert_eq!(5, *r2);
        }
        assert_eq!(5, n);
    }

    #[test]
    fn arbitrary_memory_address() {
        let p = 0x012345usize;
        let _r = p as *const i32;
        // not dereferencing that
    }

    #[test]
    fn function_with_unsafe_block() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v;

        let (xs, ys) = split(r, 3);

        assert_eq!(*xs, [1, 2, 3]);
        assert_eq!(*ys, [4, 5, 6]);
    }

    #[test]
    fn lifetime_subtyping() {
        let str = String::from("Hello lifetime");
        let ctx = Context(&str);

        let result = parse_context(ctx);

        assert!(result.is_err());
    }

    #[test]
    fn inference_trait_lifetime() {
        let name = String::from("Jack");

        let _jack = Box::new(Dog { name: &name }) as Box<dyn Terrier>;
    }

    #[test]
    fn placeholder_types() {
        let counter = Counter::new();

        let result: u32 = counter.sum();

        assert_eq!((11 * 10) / 2, result);
    }

    #[test]
    fn type_alias() {
        type Magic = i32;

        let x: Magic = 42;
        let y: i32 = 8;

        assert_eq!(42 + 8, x + y);
    }
}

use std::slice;
fn split(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let p = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(p, mid),
            slice::from_raw_parts_mut(p.offset(mid as isize), len - mid),
        )
    }
}

struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

trait Terrier {}

struct Dog<'a> {
    name: &'a str,
}

impl<'a> Terrier for Dog<'a> {}

#[derive(PartialEq, Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.count {
            0...9 => {
                self.count += 1;
                Some(self.count)
            }
            _ => None,
        }
    }
}
