struct Solution;
use std::cmp::Ordering::{Equal,Less};
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0,nums.len());
        while l < r {
            let m = l + (r-l)/2;
            match nums[m].cmp(&target) {
                Equal => return m as i32,
                Less => l=m+1,
                _ => r=m,
            }
        }
        l as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![1,3,5,6];
        let target = 5;
        let ans = 2;
        let result = Solution::search_insert(nums,target);
        assert_eq!(result, ans);
    }

    #[test]
    fn t2() {
        let nums = vec![1,3,5,6];
        let target = 2;
        let ans = 1;
        let result = Solution::search_insert(nums,target);
        assert_eq!(result, ans);
    }

    #[test]
    fn t3() {
        let nums = vec![1,3,5,6];
        let target = 7;
        let ans = 4;
        let result = Solution::search_insert(nums,target);
        assert_eq!(result, ans);
    }

}
