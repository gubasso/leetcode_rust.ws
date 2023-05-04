use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {

        let set: HashSet<char> = sentence.chars().collect();

        const ASCII_LOWER: [char; 26] = [
            'a', 'b', 'c', 'd', 'e',
            'f', 'g', 'h', 'i', 'j',
            'k', 'l', 'm', 'n', 'o',
            'p', 'q', 'r', 's', 't',
            'u', 'v', 'w', 'x', 'y',
            'z',
        ];

        for ch in ASCII_LOWER {
            if !set.contains(&ch) { return false; }
        }

        true

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::check_if_pangram("thequickbrownfoxjumpsoverthelazydog".into());
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let result = Solution::check_if_pangram("leetcode".into());
        assert_eq!(result, false);
    }

}
