use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {

        let mut dict: HashMap<char, i32> = HashMap::new();

        for m in magazine.chars() {
            if let Some(total) = dict.get_mut(&m) {
                *total += 1;
            } else {
                dict.insert(m, 1);
            }
        }

        for r in ransom_note.chars() {
            if let Some(total) = dict.get_mut(&r) {
                if *total == 0 {
                    return false;
                }
                *total -= 1;
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
