use std::iter::FromIterator;
use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut set: HashSet<char> = HashSet::from_iter(jewels.chars());
        stones.chars().fold(0, |acc, s| {
            if set.contains(&s) { acc + 1 } else { acc + 0 }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::num_jewels_in_stones("aA".to_owned(), "aAAbbbb".to_owned());
        assert_eq!(result, 3);
    }

    #[test]
    fn t2() {
        let result = Solution::num_jewels_in_stones("z".to_owned(), "ZZ".to_owned());
        assert_eq!(result, 0);
    }

}
