use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let set: HashSet<i32> = nums.into_iter().collect();
        (0..=n).skip_while(|e| set.contains(e)).next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::missing_number(vec![3,0,1]);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let result = Solution::missing_number(vec![0,1]);
        assert_eq!(result, 2);
    }

    #[test]
    fn t3() {
        let result = Solution::missing_number(vec![9,6,4,2,3,5,7,0,1]);
        assert_eq!(result, 8);
    }

}
