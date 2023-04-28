struct Solution;
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        println!("\n");
        let max_pos = nums.len() - 1;
        let mut i = 0;
        let mut j = max_pos;
        let mut idx;
        let mut sqr = vec![0; nums.len()];

        for k in (0..=max_pos).rev() {
            if nums[i].abs() < nums[j].abs() {
                idx = j;
                j -= 1;
            } else {
                idx = i;
                i += 1;
            }
            sqr[k] = nums[idx].pow(2);
        }

        sqr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::sorted_squares(vec![-4,-1,0,3,10]);
        assert_eq!(result, vec![0,1,9,16,100]);
    }

    #[test]
    fn t2() {
        let result = Solution::sorted_squares(vec![-7,-3,2,3,11]);
        assert_eq!(result, vec![4,9,9,49,121]);
    }

}
