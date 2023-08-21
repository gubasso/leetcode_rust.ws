struct Solution;
impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut ans = 1;
        let mut x = nums[0];
        for i in 1..nums.len() {
            if nums[i] - x > k {
                ans += 1;
                x = nums[i];
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![3,6,1,2,5];
        let k = 2;
        let output = 2;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn t2() {
        let nums = vec![1,2,3];
        let k = 1;
        let output = 2;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn t3() {
        let nums = vec![2,2,4,5];
        let k = 0;
        let output = 3;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, output);
    }

}
