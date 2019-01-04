#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_handles_32() {
        assert_eq!(to_celsius(32.0), 0.0);
    }
    
    #[test]
    fn it_handles_50() {
        assert_eq!(to_celsius(50.0), 10.0);
    }
    
    #[test]
    fn it_handles_212() {
        assert_eq!(to_celsius(212.0), 100.0);
    }
    
    #[test]
    fn it_handles_negative_26_5() {
        assert_eq!(to_celsius(-26.5), -32.5);
    }
}

fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}