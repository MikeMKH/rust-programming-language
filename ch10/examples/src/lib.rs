#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_largest_copy() {
        let list = vec![1, 9, 3];
        assert_eq!(largest_copy(&list), 9);
    }

    #[test]
    fn it_finds_largest_clone() {
        let list = vec![1, 9, 3];
        assert_eq!(largest_clone(&list), 9);
    }

    #[test]
    fn it_finds_largest_lifetime() {
        let list = vec![1, 9, 3];
        assert_eq!(*largest_lifetime(&list), 9);
    }

    #[test]
    fn barrow_checker_example() {
        let r;
        {
            let x = 5;
            r = &x;

            assert_eq!(*r, 5);
        } // `x` dropped here while still borrowed
          // assert_eq!(*r, 5); // borrowed value does not live long enough
    }

    #[test]
    fn it_will_find_the_longest() {
        let long = String::from("this is the longest");
        let short = "shortest";

        assert_eq!(longest(long.as_str(), short), long.as_str());
    }

    #[test]
    fn it_will_find_the_longest_using_the_smaller_scope() {
        let long;
        let short = "shortest";
        {
            long = String::from("this is the longest");

            assert_eq!(longest(long.as_str(), short), long.as_str());
        }
        // assert_eq!(longest(long.as_str(), short), long.as_str()); // not found in this scope
    }
}

fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

fn largest_lifetime<'a, T: PartialOrd>(list: &'a [T]) -> &'a T {
    let mut largest = &list[0];
    let mut largest_idx = 0;

    for (i, ref item) in list.iter().enumerate() {
        if *item > largest {
            largest = &list[i];
            largest_idx = i;
        }
    }

    &list[largest_idx]
}

use std::cmp::Ordering;
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    match x.len().cmp(&y.len()) {
        Ordering::Greater => x,
        Ordering::Equal => x,
        Ordering::Less => y,
    }
    /*
    -- what normal people would do
        if x.len() > y.len() {
            x
        } else {
            y
        }
    */
}
