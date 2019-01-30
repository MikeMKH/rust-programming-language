//! Succ2
//!
//! A library for finding the successor's successor.

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

/// Finds the successor of the successor of the number given.
///
/// # Example
///
/// ```
/// let five = 5;
///
/// assert_eq!(7, succ(five))
///```
pub fn succ2(x: u32) -> u32 {
    x + 2
}
