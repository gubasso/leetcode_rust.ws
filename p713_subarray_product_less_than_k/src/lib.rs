struct Solution;
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 { return 0; }
        let (mut i, mut j, mut multip, mut ans) = (0, 0, 1, 0);

        for (j, n) in nums.iter().enumerate() {
            multip *= n;
            while multip >= k && i <= j {
                multip /= nums[i];
                i += 1;
            }
            ans += j as i32 - i as i32 + 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::num_subarray_product_less_than_k(vec![10,5,2,6], 100);
        assert_eq!(result, 8);
    }

    #[test]
    fn t2() {
        let result = Solution::num_subarray_product_less_than_k(vec![1,2,3], 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn t3() {
        let result = Solution::num_subarray_product_less_than_k(vec![1,1,1], 1);
        assert_eq!(result, 0);
    }

}
