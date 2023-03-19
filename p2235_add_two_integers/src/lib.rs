pub fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = sum(12, 5);
        assert_eq!(result, 17);
    }

    #[test]
    fn t2() {
        let result = sum(-10, 4);
        assert_eq!(result, -6);
    }

}

