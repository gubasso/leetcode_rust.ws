struct Solution;
impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 { return nums }
        let total_elem: i64 = 2*k as i64 + 1;
        let n = nums.len();
        let q = k as usize;
        let mut avgs: Vec<i32> = vec![-1; n];
        if total_elem as usize > n { return avgs }

        let mut prefix = vec![nums[0] as i64];
        for i in 1..n {
            prefix.push(prefix[prefix.len() - 1] + nums[i] as i64);
        }

        for i in q..n-q {
            let i_radius = i - q;
            let j_radius = i + q;
            let sum = prefix[j_radius] - prefix[i_radius] + nums[i_radius] as i64;
            avgs[i] = (sum/total_elem) as i32;
        }

        avgs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::get_averages(vec![7,4,3,9,1,8,5,2,6], 3);
        assert_eq!(result, vec![-1,-1,-1,5,4,4,-1,-1,-1]);
    }

    #[test]
    fn t2() {
        let result = Solution::get_averages(vec![100000], 0);
        assert_eq!(result, vec![100000]);
    }

    #[test]
    fn t3() {
        let result = Solution::get_averages(vec![8], 100000);
        assert_eq!(result, vec![-1]);
    }

}
