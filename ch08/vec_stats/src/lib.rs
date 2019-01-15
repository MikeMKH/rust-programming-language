// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_mean_for_no_elements() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(mean(v), 0.0);
    }

    #[test]
    fn it_finds_mean_for_an_element() {
        let v: Vec<i32> = vec![8];
        assert_eq!(mean(v), 8.0);
    }

    #[test]
    fn it_finds_mean_for_many_elements() {
        let v: Vec<i32> = vec![1, 2, 3, 4, 0];
        assert_eq!(mean(v), 2.0);
    }

    #[test]
    fn it_finds_median_for_no_elements() {
        let mut v: Vec<i32> = Vec::new();
        assert_eq!(median(&mut v), None);
    }

    #[test]
    fn it_finds_median_for_an_element() {
        let mut v: Vec<i32> = vec![1];
        assert_eq!(median(&mut v).unwrap(), 1);
    }

    #[test]
    fn it_finds_median_for_even_number_of_elements() {
        let mut v: Vec<i32> = vec![1, 3];
        assert_eq!(median(&mut v).unwrap(), 2);
    }

    #[test]
    fn it_finds_median_for_odd_number_of_elements() {
        let mut v: Vec<i32> = vec![7, 3, 9, 1, 5];
        assert_eq!(median(&mut v).unwrap(), 5);
    }
}

fn mean(v: Vec<i32>) -> f64 {
    let n: i32 = v.iter().sum();
    match n {
        0 => 0.0,
        _ => n as f64 / v.len() as f64,
    }
}

fn median(v: &mut Vec<i32>) -> Option<i32> {
    v.sort();
    match v.len() == 0 {
        true => None,
        false => {
            let midpoint = v.len() / 2;
            match v.len() % 2 == 0 {
                true => Some((v[midpoint] + v[midpoint - 1]) / 2),
                false => v.get(midpoint).cloned(),
            }
        }
    }
}
