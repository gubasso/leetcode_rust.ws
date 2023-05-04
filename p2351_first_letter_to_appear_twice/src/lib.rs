use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut chars = HashSet::new();
        s.chars().skip_while(|&c| chars.insert(c)).next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::repeated_character("abccbaacz".into());
        assert_eq!(result, 'c');
    }

    #[test]
    fn t2() {
        let result = Solution::repeated_character("abcdd".into());
        assert_eq!(result, 'd');
    }

}
