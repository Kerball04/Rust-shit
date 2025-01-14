fn add_number(num1: u32, num2: u32) -> u32 {
    num1 + num2
}

fn div_number(num1: f32, num2: f32) -> f32 {
    if num2 == 0.0 {
        -1.0
    } else {
        num1 / num2
    }
}

fn mul_number(num1: u32, num2: u32) -> u32 {
    num1 * num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_number() {
        assert_eq!(add_number(4, 1), 5);
    }

    #[test]
    fn test_div_number() {
        assert_eq!(div_number(1.0, 4.0), 0.25);
    }

    #[test]
    fn test_mul_number() {
        assert_eq!(mul_number(2, 4), 8);
    }
}

fn main() {
    println!("Add 4 and 1: {}", add_number(4, 1));
    println!("Divide 1 and 4: {}", div_number(1.0, 4.0));
    println!("Multiply 2 and 4: {}", mul_number(2, 4));
}
