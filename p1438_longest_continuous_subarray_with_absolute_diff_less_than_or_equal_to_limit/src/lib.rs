use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let (mut ans, mut l) = (0,0);
        let mut inc = VecDeque::new();
        let mut dec = VecDeque::new();

        for r in 0..nums.len() {
            while inc.back().map_or(false, |&n| nums[r] < n) {
                inc.pop_back();
            }
            while dec.back().map_or(false, |&n| nums[r] > n) {
                dec.pop_back();
            }
            inc.push_back(nums[r]);
            dec.push_back(nums[r]);

            while dec.front().unwrap() - inc.front().unwrap() > limit {
                if dec.front().is_some() && nums[l] == *dec.front().unwrap() {
                    dec.pop_front();
                }
                if inc.front().is_some() && nums[l] == *inc.front().unwrap() {
                    inc.pop_front();
                }

                l += 1;
            }
            ans = ans.max(r - l + 1);

        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::longest_subarray(vec![8,2,4,7], 4);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let result = Solution::longest_subarray(vec![10,1,2,4,7,2], 5);
        assert_eq!(result, 4);
    }

    #[test]
    fn t3() {
        let result = Solution::longest_subarray(vec![4,2,2,2,4,4,2,2], 0);
        assert_eq!(result, 3);
    }

}
