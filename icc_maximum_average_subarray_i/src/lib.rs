struct Solution;
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut curr_avg: f64 = 0.0;
        let mut i: usize = 0;

        for i in 0..k as usize {
            curr_avg += nums[i] as f64 / k as f64 ;
        }
        let mut max_avg: f64 = curr_avg;

        for j in k as usize..nums.len(){
            let avg_rem = nums[i] as f64 / k as f64;
            let avg_add = nums[j] as f64 / k as f64;
            curr_avg = curr_avg - avg_rem + avg_add;
            i = j - (k as usize - 1);
            max_avg = f64::max(max_avg, curr_avg);
        }

        max_avg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::find_max_average(vec![1,12,-5,-6,50,3], 4);
        assert_eq!(result, 12.75000);
    }

    #[test]
    fn t2() {
        let result = Solution::find_max_average(vec![5], 1);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn t3() {
        let result = Solution::find_max_average(vec![0,4,0,3,2], 1);
        assert_eq!(result, 4.0);
    }

}
