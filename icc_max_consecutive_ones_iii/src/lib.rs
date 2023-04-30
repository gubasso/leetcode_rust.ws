struct Solution;
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut zeros: i32 = 0;
        let mut max: i32 = 0;
        let mut i: usize = 0;

        for j in 0..nums.len() {
            if nums[j] == 0 { zeros += 1; }

            while zeros > k {
                if nums[i] == 0 { zeros -= 1 }
                i += 1;
            }
            max = i32::max(max, j as i32 - i as i32 + 1);
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::longest_ones(vec![1,1,1,0,0,0,1,1,1,1,0], 2);
        assert_eq!(result, 6);
    }

    #[test]
    fn t2() {
        let result = Solution::longest_ones(vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3);
        assert_eq!(result, 10);
    }

}
