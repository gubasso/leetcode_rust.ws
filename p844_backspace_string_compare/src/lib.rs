fn build_stack(s: &String) -> String {
    let mut stack = Vec::new();
    for c in s.chars() {
        if c == '#' {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack.into_iter().collect()
}

struct Solution;
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        build_stack(&s).eq(&build_stack(&t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let result = Solution::backspace_compare("ab##".to_string(), "c#d#".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn t3() {
        let result = Solution::backspace_compare("a#c".to_string(), "b".to_string());
        assert_eq!(result, false);
    }

}
