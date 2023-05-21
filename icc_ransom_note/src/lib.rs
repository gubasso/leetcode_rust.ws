use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if magazine.len() == 0 { return false; };
        let mut map: HashMap<char, u32> = HashMap::new();

        for m in magazine.chars() {
            map.entry(m).and_modify(|c| *c += 1 ).or_insert(1);
        }

        for r in ransom_note.chars() {
            if let Some(c) = map.get_mut(&r) {
                if *c == 0 { return false; }
                *c -= 1;
            } else {
                return false;
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
        let result = Solution::can_construct("a".to_owned(), "b".to_owned());
        assert_eq!(result, false);
    }

    #[test]
    fn t2() {
        let result = Solution::can_construct("aa".to_owned(), "ab".to_owned());
        assert_eq!(result, false);
    }

    #[test]
    fn t3() {
        let result = Solution::can_construct("aa".to_owned(), "aab".to_owned());
        assert_eq!(result, true);
    }

}
