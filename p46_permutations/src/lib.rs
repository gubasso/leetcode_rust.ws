struct Solution;
impl Solution {
    fn backtrack(res: &mut Vec<Vec<i32>>, nums: &[i32], sub: &[i32]) {
        if nums.len() == 0 {
            res.push(Vec::from(sub.clone()));
            return;
        }
        for i in 0..nums.len() {
            let (mut nums_c, mut sub_c) = (nums.to_vec(), sub.to_vec());
            sub_c.push(nums_c.remove(i));
            Self::backtrack(res, &nums_c, &sub_c);
        }
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sub = Vec::with_capacity(nums.len());
        let mut res = vec![];
        Self::backtrack(&mut res, &mut nums, &mut sub);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![1,2,3];
        let output = vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],vec![2,3,1],vec![3,1,2],vec![3,2,1]];
        let result = Solution::permute(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn t2() {
        let nums = vec![0,1];
        let output = vec![vec![0,1],vec![1,0]];
        let result = Solution::permute(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn t3() {
        let nums = vec![1];
        let output = vec![vec![1]];
        let result = Solution::permute(nums);
        assert_eq!(result, output);
    }

}
