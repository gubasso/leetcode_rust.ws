struct Solution;
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum: i32 = 0;
        let mut sum_vec: Vec<i32> = Vec::new();
        for e in nums {
            sum += e;
            sum_vec.push(sum);
        }
        sum_vec
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::running_sum(vec![1,2,3,4]);
        assert_eq!(result, vec![1,3,6,10]);
    }

    #[test]
    fn t2() {
        let result = Solution::running_sum(vec![1,1,1,1,1]);
        assert_eq!(result, vec![1,2,3,4,5]);
    }

    #[test]
    fn t3() {
        let result = Solution::running_sum(vec![3,1,2,10,1]);
        assert_eq!(result, vec![3,4,6,16,17]);
    }

}
