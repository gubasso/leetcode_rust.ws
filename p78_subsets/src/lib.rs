struct Solution;
impl Solution {

    fn backtrack(nums: &Vec<i32>, ans: &mut Vec<Vec<i32>>, mut curr: Vec<i32>, i: usize) {
        if i > nums.len() {
            return;
        }
        ans.push(curr.to_vec());
        for j in i..nums.len() {
            curr.push(nums[j]);
            Self::backtrack(nums, ans, curr.to_vec(), j + 1);
            curr.pop();
        }
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // println!("\n");
        let mut ans = vec![];
        Self::backtrack(&nums, &mut ans, vec![], 0);
        ans.sort_by_key(|v| v.len());
        ans
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![1,2,3];
        let mut out = vec![vec![],vec![1],vec![2],vec![1,2],vec![3],vec![1,3],vec![2,3],vec![1,2,3]];
        out.sort_by_key(|v| v.len());
        let result = Solution::subsets(nums);
        assert_eq!(result, out);
    }

    #[test]
    fn t2() {
        let nums = vec![0];
        let out = vec![vec![],vec![0]];
        let result = Solution::subsets(nums);
        assert_eq!(result, out);
    }

}
