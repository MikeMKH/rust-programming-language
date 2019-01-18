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
