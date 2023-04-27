struct Solution;
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut i: usize = 0;
        let mut j: usize = s.len()-1;

        while i != j && i < j {
            let left = s[i];
            let right = s[j];
            s[i] = right;
            s[j] = left;
            i += 1; j -= 1;
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let mut s: Vec<char> = "hello".chars().collect();
        Solution::reverse_string(&mut s);
        assert_eq!(s, "olleh".chars().collect::<Vec<char>>());
    }

    #[test]
    fn t2() {
        let mut s: Vec<char> = "Hannah".chars().collect();
        Solution::reverse_string(&mut s);
        assert_eq!(s, "hannaH".chars().collect::<Vec<char>>());
    }

}
