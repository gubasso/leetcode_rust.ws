struct Solution;

use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq, Debug)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        // println!("\n");
        let mut count = 0;
        let mut sum = 0.0;
        let nums: Vec<MinNonNan> = nums.into_iter().map(|n| {
            let n = n as f64;
            sum += n;
            MinNonNan(n)
        }).collect();
        let half = sum/2.0;
        let mut heap: BinaryHeap<MinNonNan> = BinaryHeap::from(nums);

        while half < sum {
            let half_n = heap.pop().unwrap().0 / 2.0;
            sum -= half_n;
            heap.push(MinNonNan(half_n));
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![5,19,8,1];
        let result = Solution::halve_array(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn t2() {
        let nums = vec![3,8,20];
        let result = Solution::halve_array(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn t3() {
        let nums = vec![1];
        let result = Solution::halve_array(nums);
        assert_eq!(result, 1);
    }

}
