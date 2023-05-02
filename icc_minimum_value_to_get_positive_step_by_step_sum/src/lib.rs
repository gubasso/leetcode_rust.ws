struct Solution;
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut last_sum = 0;
        let mut min: i32 = 0;

        for i in 0..nums.len() {
            last_sum += nums[i];
            min = i32::min(min, last_sum);
        }

        - min + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::min_start_value(vec![-3,2,-3,4,2]);
        assert_eq!(result, 5);
    }

    #[test]
    fn t2() {
        let result = Solution::min_start_value(vec![1,2]);
        assert_eq!(result, 1);
    }

    #[test]
    fn t3() {
        let result = Solution::min_start_value(vec![1,-2,-3]);
        assert_eq!(result, 5);
    }

    #[test]
    fn t4() {
        let result = Solution::min_start_value(vec![-3,6,2,5,8,6]);
        assert_eq!(result, 4);
    }

}
