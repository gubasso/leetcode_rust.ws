use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for c in s.chars() {
            match c {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                closing => if Some(closing) != stack.pop() { return false },
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::is_valid("()".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let result = Solution::is_valid("()[]{}".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn t3() {
        let result = Solution::is_valid("(]".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn t4() {
        let result = Solution::is_valid("({)}".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn t5() {
        let result = Solution::is_valid("]".to_string());
        assert_eq!(result, false);
    }

}
