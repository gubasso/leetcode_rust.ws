use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 0;
        let mut left: i32 = -1;
        for (i, ch) in s.chars().enumerate() {
            let right = i as i32;
            if let Some(last_saved) = map.insert(ch, right) {
                left = left.max(last_saved);
            }
            ans = ans.max(right - left);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::length_of_longest_substring("abcabcbb".to_owned());
        assert_eq!(result, 3);
    }

    #[test]
    fn t2() {
        let result = Solution::length_of_longest_substring("bbbbb".to_owned());
        assert_eq!(result, 1);
    }

    #[test]
    fn t3() {
        let result = Solution::length_of_longest_substring("pwwkew".to_owned());
        assert_eq!(result, 3);
    }

    #[test]
    fn t4() {
        let result = Solution::length_of_longest_substring("wpwkew".to_owned());
        assert_eq!(result, 4);
    }

}
