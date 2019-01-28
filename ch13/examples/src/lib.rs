#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demostration() {
        let v = vec![1, 2, 3];

        let mut iter = v.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v = vec![1, 2, 3];

        let total: i32 = v.iter().sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map() {
        let v = vec![1, 2, 3];

        let m: Vec<_> = v.iter().map(|x| x + 1).collect();

        assert_eq!(m, vec![2, 3, 4]);
    }

    #[test]
    fn iterator_filer() {
        let v = vec![
            Shoe {
                size: 12,
                style: String::from("vans"),
            },
            Shoe {
                size: 10,
                style: String::from("heels"),
            },
            Shoe {
                size: 12,
                style: String::from("brown dress"),
            },
        ];

        let my_size: u8 = 12;
        let in_my_size: Vec<_> = v
            .iter()
            .filter(|s| s.size == my_size)
            .map(|s| s.style.clone())
            .collect();

        assert_eq!(in_my_size, vec!["vans", "brown dress"]);
    }

    #[test]
    fn calling_next() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn witness_the_power_of_street_knownledge() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(x, y)| x * y)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(sum, 18);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u8,
    style: String,
}

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
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
