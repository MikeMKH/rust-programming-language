// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_mean_for_empty() {
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
}

fn mean(v: Vec<i32>) -> f64 {
    let n: i32 = v.iter().sum();
    match n {
        0 => 0.0,
        _ => n as f64 / v.len() as f64,
    }
}
