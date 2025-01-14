pub fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2
}

pub fn sub(n1: i32, n2: i32) -> i32 {
    return n1 - n2
}

pub fn div(n1: f32, n2: f32) -> f32 {
    return n1 / n2
}

pub fn mul(n1: f32, n2:f32) -> f32 {
    return n1 * n2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(4, 1), 5);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(4, 1), 3);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(2.0, 5.0), 0.4);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(3.0, 3.0), 9.0);
    }
}