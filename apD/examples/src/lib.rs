#[cfg(test)]
mod tests {
    macro_rules! vector {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
    
    macro_rules! ones {
        ( $( $x:expr )*) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    for i in 0..$x {
                        temp_vec.push(1);
                    }
                )*
                temp_vec
            }
        };
    }

    #[test]
    fn declarative_macro() {
        let v: Vec<i32> = vector![1, 2, 3];

        assert_eq!(v, vec![1, 2, 3])
    }

    #[test]
    fn it_makes_1_ones() {
        let v: Vec<i32> = ones![1];

        assert_eq!(v, vec![1])
    }

    #[test]
    fn it_makes_3_ones() {
        let v: Vec<i32> = ones![3];

        assert_eq!(v, vec![1, 1, 1])
    }
}
