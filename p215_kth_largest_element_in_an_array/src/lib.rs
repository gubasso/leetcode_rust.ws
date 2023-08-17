struct Solution;
use std::{collections::BinaryHeap, cmp::Reverse};
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // println!("\n");
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(nums[0]));

        for n in nums.into_iter().skip(1) {
            if heap.len() < k as usize {
                heap.push(Reverse(n));
                continue;
            }
            let curr = heap.peek().unwrap().0;
            if n > curr {
                heap.pop();
                heap.push(Reverse(n));
            }
        }

        heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![3,2,1,5,6,4];
        let k = 2;
        let result = Solution::find_kth_largest(nums, k);
        assert_eq!(result, 5);
    }

    #[test]
    fn t2() {
        let nums = vec![3,2,3,1,2,4,5,5,6];
        let k = 4;
        let result = Solution::find_kth_largest(nums, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn t3() {
        let nums = vec![2,1];
        let k = 2;
        let result = Solution::find_kth_largest(nums, k);
        assert_eq!(result, 1);
    }

}
