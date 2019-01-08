#[cfg(test)]
mod tests {
    #[test]
    fn variable_scope() {
        let s = "out";
        {
            let s = "in";
            assert_eq!(s, "in");
        }
        assert_eq!(s, "out");
    }
    
    #[test]
    fn string_type() {
        let mut s = String::from("hello");
        assert_eq!(s, "hello");
        
        s.push_str(" world");
        assert_eq!(s, "hello world");
    }
    
    #[test]
    fn move_value_type() {
        let x = 5;
        let y = x;
        
        assert_eq!(x, 5);
        assert_eq!(y, 5);
        
        {
            let x = 6;
            
            assert_eq!(x, 6);
            assert_eq!(y, 5);
        }
        
        assert_eq!(x, 5);
        assert_eq!(y, 5);
    }
    
    #[test]
    fn move_reference_type() {
        let mut x = String::from("hi");
        let mut y = x;
        
        // assert_eq!(x, "hi"); // error[E0382]: borrow of moved value: `x`
        assert_eq!(y, "hi");
        
        // x.push_str(" there"); // error[E0382]: borrow of moved value: `x`
        y.push_str(" there");
        
        // assert_eq!(x, "hi"); // error[E0382]: borrow of moved value: `x`
        assert_eq!(y, "hi there");
    }
    
    #[test]
    fn clone_reference_type() {
        let mut x = String::from("hello");
        let mut y = x.clone();
        
        assert_eq!(x, "hello");
        assert_eq!(y, "hello");
        
        x.push_str(" Mike");
        y.push_str(" Kelsey");
        
        assert_eq!(x, "hello Mike");
        assert_eq!(y, "hello Kelsey");
    }
    
    #[test]
    fn copy_value_type() {
        let x = 5;
        let y = x;
        
        assert_eq!(x, 5);
        assert_eq!(y, 5);
    }
}
