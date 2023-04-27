// Time complexity: O(n)
// Space complexity: O(1)
struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut is: usize = 0;
        let mut it: usize = 0;
        let t_vec: Vec<char> = t.chars().collect();
        let s_vec: Vec<char> = s.chars().collect();

        while it < t.len() && is < s.len() {
            if t_vec[it] == s_vec[is] { is += 1; };
            it += 1
        };
        is == s.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::is_subsequence("abc".into(), "ahbgdc".into());
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let result = Solution::is_subsequence("axc".into(), "ahbgdc".into());
        assert_eq!(result, false);
    }

}
