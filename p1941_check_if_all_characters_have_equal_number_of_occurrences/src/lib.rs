use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        let mut map: HashMap<char, i32> = HashMap::new();

        for ch in s.chars() {
            map.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
        }

        for (_,v) in map {
            set.insert(v);
        }

        set.len() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::are_occurrences_equal("abacbc".into());
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let result = Solution::are_occurrences_equal("aaabb".into());
        assert_eq!(result, false);
    }

}
