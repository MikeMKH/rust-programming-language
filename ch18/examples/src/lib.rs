#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_let() {
        let favorite_color: Option<&str> = None;

        if let Some(color) = favorite_color {
            assert_eq!(color, "Green");
        }

        assert_eq!(None, favorite_color);
    }

    #[test]
    fn while_let() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut expected = 3;
        while let Some(top) = stack.pop() {
            assert_eq!(expected, top);
            expected -= 1;
        }

        assert_eq!(0, stack.len());
    }

    #[test]
    fn for_loop() {
        let v = vec!["Kelsey", "Jack", "Mike"];
        let mut visited = Vec::new();

        let mut expected = 0;
        for (index, name) in v.iter().enumerate() {
            assert_eq!(expected, index);

            expected += 1;
            visited.push(*name);
        }

        assert_eq!(v, visited);
    }
}
