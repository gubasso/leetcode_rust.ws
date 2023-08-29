struct Solution;
impl Solution {

    fn backtrack(candidates: &Vec<i32>, target: i32,
                 ans: &mut Vec<Vec<i32>>,
                 path: &mut Vec<i32>, start_idx: usize,
                 curr_sum: i32) {

        if curr_sum == target {
            ans.push(path.to_vec());
            return;
        }

        for i in start_idx..candidates.len() {
            let num = candidates[i];
            let sum = curr_sum + num;
            if sum <= target {
                path.push(num);
                Self::backtrack(candidates, target, ans, path, i, sum);
                path.pop();
            }
        }

    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut path = vec![];
        Self::backtrack(&candidates, target, &mut ans, &mut path, 0, 0);
        ans
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let candidates = vec![2,3,6,7];
        let target = 7;
        let out = vec![vec![2,2,3],vec![7]];
        let result = Solution::combination_sum(candidates,target);
        assert_eq!(result, out);
    }

    #[test]
    fn t2() {
        let candidates = vec![2,3,5];
        let target = 8;
        let out = vec![vec![2,2,2,2],vec![2,3,3],vec![3,5]];
        let result = Solution::combination_sum(candidates,target);
        assert_eq!(result, out);
    }

    #[test]
    fn t3() {
        let candidates = vec![2];
        let target = 1;
        let out: Vec<Vec<i32>> = vec![];
        let result = Solution::combination_sum(candidates,target);
        assert_eq!(result, out);
    }

}
