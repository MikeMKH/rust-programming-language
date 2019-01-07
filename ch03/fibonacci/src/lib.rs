#[macro_use] extern crate cached;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_returns_0_for_0() {
        assert_eq!(fibonacci(0), 0);
    }
    
    #[test]
    fn it_returns_1_for_1() {
        assert_eq!(fibonacci(1), 1);
    }
    
    #[test]
    fn it_returns_1_for_2() {
        assert_eq!(fibonacci(2), 1);
    }
    
    #[test]
    fn it_returns_2_for_3() {
        assert_eq!(fibonacci(3), 2);
    }
    
    #[test]
    fn it_returns_6765_for_20() {
        assert_eq!(fibonacci(20), 6765);
    }
    
    #[test]
    fn it_returns_2971215073_for_47() {
        assert_eq!(fibonacci(47), 2971215073);
    }
}

cached! {
    FIB;
    fn fibonacci(n: u32) -> u32 = {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 2) + fibonacci(n - 1)
        }
    }
}