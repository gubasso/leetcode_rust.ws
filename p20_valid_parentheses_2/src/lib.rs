#[cfg(test)]
mod tests {
    struct Solution;

    impl Solution {
        pub fn is_valid(s: String) -> bool {
            let mut stack = Vec::new();
            for c in s.chars() {
                match c {
                    '(' => stack.push(')'),
                    '{' => stack.push('}'),
                    '[' => stack.push(']'),
                    ')' | '}' | ']' if Some(c) != stack.pop() => return false,
                    _ => (),
                }
            }
            stack.is_empty()
        }
    }

    #[test]
    fn t1() {
        let result = Solution::is_valid("()".to_string());
        assert!(result);
    }

    #[test]
    fn t2() {
        let result = Solution::is_valid("()[]{}".to_string());
        assert!(result);
    }

    #[test]
    fn t3() {
        let result = Solution::is_valid("(]".to_string());
        assert!(!result);
    }

    #[test]
    fn t4() {
        let result = Solution::is_valid("(])".to_string());
        assert!(!result);
    }

}
