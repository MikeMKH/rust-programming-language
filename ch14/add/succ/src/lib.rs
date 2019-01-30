#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(succ(0), 1);
    }
    
    #[test]
    fn it_really_works() {
        assert_eq!(succ(succ(succ(0))), 3);
    }
}

pub fn succ(x: u32) -> u32 {
    x + 1
}
