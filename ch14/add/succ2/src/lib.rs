#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(succ2(2), 4);
    }
    
    #[test]
    fn it_really_works() {
        assert_eq!(succ2(succ2(succ2(0))), 6);
    }
}

pub fn succ2(x: u32) -> u32 {
    x + 2
}
