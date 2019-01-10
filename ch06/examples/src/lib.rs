use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_display_an_ip4() {
        let home = IpAddr::V4(127, 0, 0, 1);
        assert_eq!(format!("{}", home), "127.0.0.1");
    }

    #[test]
    fn it_can_display_an_ip6() {
        let loopback = IpAddr::V6(String::from("::1"));
        assert_eq!(format!("{}", loopback), "::1");
    }

    #[test]
    fn it_can_add_some_some() {
        assert_eq!(add_option(Some(2), Some(3)), Some(5));
    }

    #[test]
    fn it_can_add_none_some() {
        assert_eq!(add_option(None, Some(3)), None);
    }

    #[test]
    fn it_can_add_some_none() {
        assert_eq!(add_option(Some(2), None), None);
    }

    #[test]
    fn it_can_add_none_none() {
        assert_eq!(add_option(None, None), None);
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl std::fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            IpAddr::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(ref addr) => addr.clone(),
        };
        write!(f, "{}", printable)
    }
}

fn add_option(left: Option<u32>, right: Option<u32>) -> Option<u32> {
    match left {
        Some(n) => match right {
            Some(m) => Some(n + m),
            None => None,
        },
        None => None,
    }
}
