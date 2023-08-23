struct Solution;
impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let check = |divisor: i32| -> bool {
            let mut sum = 0;
            for &n in nums.iter() {
                sum += (n / divisor) + (n % divisor).signum();
                if sum > threshold {
                    return false;
                }
            }
            true
        };

        let (mut l, mut r) = (1, *nums.iter().max().unwrap());

        while l < r {
            let div = l + (r-l)/2;
            match check(div) {
                false => l=div+1,
                true => r=div,
            }
        }

        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![1,2,5,9];
        let threshold = 6;
        let result = Solution::smallest_divisor(nums,threshold);
        assert_eq!(result, 5);
    }

    #[test]
    fn t2() {
        let nums = vec![44,22,33,11,1];
        let threshold = 5;
        let result = Solution::smallest_divisor(nums,threshold);
        assert_eq!(result, 44);
    }

}
