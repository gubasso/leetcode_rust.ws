struct Solution;
use std::cmp::Ordering::{Equal, Less, Greater};
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0,nums.len());
        while l < r {
            let m = l + (r-l)/2;
            match nums[m].cmp(&target) {
                Equal => return m as i32,
                Less => l = m + 1,
                Greater => r = m,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![-1,0,3,5,9,12];
        let target = 9;
        let result = Solution::search(nums, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn t2() {
        let nums = vec![-1,0,3,5,9,12];
        let target = 2;
        let result = Solution::search(nums, target);
        assert_eq!(result, -1);
    }

    #[test]
    fn t3() {
        let nums = vec![5];
        let target = -5;
        let result = Solution::search(nums, target);
        assert_eq!(result, -1);
    }

}
