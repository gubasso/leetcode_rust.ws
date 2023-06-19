struct Solution;
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();

        for c in s.chars() {
            if Some(&c) == stack.last() {
                stack.pop();
            } else {
                stack.push(c);
            };
        }

        stack.into_iter().collect()

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::remove_duplicates("abbaca".to_string());
        assert_eq!(result, "ca".to_string());
    }

    #[test]
    fn t2() {
        let result = Solution::remove_duplicates("azxxzy".to_string());
        assert_eq!(result, "ay".to_string());
    }

}
