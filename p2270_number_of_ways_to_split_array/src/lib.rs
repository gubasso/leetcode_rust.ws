struct Solution;
impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut ways: i32 = 0;
        let mut sums: Vec<i64> = vec![nums[0] as i64];

        for i in 1..l {
            sums.push(sums[i-1] + nums[i] as i64);
        }

        for j in 0..l-1 {
            let sum_1 = sums[j];
            let sum_2 = sums[l-1] - sums[j+1] + nums[j+1] as i64;
            if sum_1 >= sum_2 { ways += 1; }
        }

        ways
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_big_vec() -> Vec<i32> {
        (-99999..=0).rev().collect()
    }

    #[test]
    fn t1() {
        let result = Solution::ways_to_split_array(vec![10,4,-8,7]);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let result = Solution::ways_to_split_array(vec![2,3,1,0]);
        assert_eq!(result, 2);
    }

    #[test]
    fn t3() {
        let result = Solution::ways_to_split_array(gen_big_vec());
        assert_eq!(result, 70710);
    }

}
