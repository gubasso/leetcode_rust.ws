use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() { return false; };
        let mut dict: HashMap<char, i32> = HashMap::new();

        for m in magazine.chars() {
            dict.entry(m).and_modify(|counter| *counter += 1).or_insert(1);
        }

        for r in ransom_note.chars() {
            match dict.get_mut(&r) {
                Some(n) if *n > 0 => { *n -= 1; }
                _ => { return false; }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::can_construct("a".into(), "b".into());
        assert_eq!(result, false);
    }

    #[test]
    fn t2() {
        let result = Solution::can_construct("aa".into(), "ab".into());
        assert_eq!(result, false);
    }

    #[test]
    fn t3() {
        let result = Solution::can_construct("aa".into(), "aab".into());
        assert_eq!(result, true);
    }

}
