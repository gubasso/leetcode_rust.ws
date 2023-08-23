struct Solution;

use std::cmp::Ordering::{Equal,Less};
impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        // println!("\n");
        let n = nums.len();
        nums.sort();
        let mut psum = vec![0;n];
        psum[0] = nums[0];
        for i in 1..n {
            psum[i] = psum[i-1] + nums[i];
        }
        let mut ans = Vec::with_capacity(queries.len());

        for q in queries {
            let (mut l, mut r) = (0,n);
            while l < r {
                let m = l + (r-l)/2;
                match psum[m].cmp(&q) {
                    Equal => {l=m+1; break},
                    Less => l=m+1,
                    _ => r=m,
                }
            }
            ans.push(l as i32);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![4,5,2,1];
        let queries = vec![3,10,21];
        let result = Solution::answer_queries(nums,queries);
        assert_eq!(result, vec![2,3,4]);
    }

    #[test]
    fn t2() {
        let nums = vec![2,3,4,5];
        let queries = vec![1];
        let result = Solution::answer_queries(nums,queries);
        assert_eq!(result, vec![0]);
    }

}
