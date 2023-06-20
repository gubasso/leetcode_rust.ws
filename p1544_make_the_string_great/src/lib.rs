struct Solution;
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            let last = if let Some(&l) = stack.last() { l } else { ' ' };
            // let last_up: char  = if let Some(l) = last.to_uppercase().collect::<Vec<char>>().pop() { l } else { ' ' };
            // let c_up: char  = if let Some(c) = c.to_uppercase().collect::<Vec<char>>().pop() { c } else { ' ' };

            if last == ' ' || last == c || last.to_uppercase().ne(c.to_uppercase()) {
                stack.push(c);
            } else {
                stack.pop();
            }

        }

        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::make_good("leEeetcode".to_string());
        assert_eq!(result, "leetcode".to_string());
    }

    #[test]
    fn t2() {
        let result = Solution::make_good("abBAcC".to_string());
        assert_eq!(result, "".to_string());
    }

    #[test]
    fn t3() {
        let result = Solution::make_good("s".to_string());
        assert_eq!(result, "s".to_string());
    }

    #[test]
    fn t4() {
        let result = Solution::make_good("kkdsFuqUfSDKK".to_string());
        assert_eq!(result, "kkdsFuqUfSDKK".to_string());
    }

}
