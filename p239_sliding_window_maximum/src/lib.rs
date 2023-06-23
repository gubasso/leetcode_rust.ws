use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut ans = Vec::with_capacity(nums.len() - k + 1);
        let mut queue = VecDeque::new();

        for i in 0..nums.len() {

            while queue.back().map_or(false, |&j| nums[i] > nums[j]) {
                queue.pop_back();
            }
            queue.push_back(i);

            if queue.front().unwrap() + k == i {
                queue.pop_front();
            }

            if i >= k - 1 {
                if let Some(&j) = queue.front() {
                    ans.push(nums[j]);
                }
                // queue.front().map(|&j| {
                //     ans.push(nums[j]);
                // });
            };

        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3);
        assert_eq!(result, vec![3,3,5,5,6,7]);
    }

    #[test]
    fn t2() {
        let result = Solution::max_sliding_window(vec![1], 1);
        assert_eq!(result, vec![1]);
    }

}
