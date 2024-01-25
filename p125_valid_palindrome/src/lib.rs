#[cfg(test)]
mod tests {
    struct Solution;
    impl Solution {
        pub fn is_palindrome(s: String) -> bool {
            let mut clear_str : String = s.chars().filter(|x| x.is_alphanumeric()).collect();
            clear_str = clear_str.to_lowercase();
            let str_cpy: String = clear_str.chars().rev().collect();
            clear_str == str_cpy
        }
    }
    #[test]
    fn t1() {
        let result = Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned());
        assert!(result);
    }

    #[test]
    fn t2() {
        let result = Solution::is_palindrome("race a car".to_owned());
        assert!(!result);
    }

    #[test]
    fn t3() {
        let result = Solution::is_palindrome(" ".to_owned());
        assert!(result);
    }

    #[test]
    fn t4() {
        let result = Solution::is_palindrome("0P".to_owned());
        assert!(!result);
    }

    #[test]
    fn t5() {
        let result = Solution::is_palindrome("a.".to_owned());
        assert!(result);
    }


}
