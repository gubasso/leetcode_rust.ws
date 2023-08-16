struct Solution;
use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        // println!("\n");
        let n = sticks.len();
        if n < 2 {
            return 0;
        }
        let sticks: Vec<Reverse<i32>> = sticks.into_iter().map(|n| Reverse(n)).collect();
        let mut heap = BinaryHeap::from(sticks);
        let mut sum = 0;
        // println!("{:?}", heap);
        while let Some(s1) = heap.pop() {
            if let Some(s2) = heap.pop() {
                let s = s1.0 + s2.0;
                sum += s;
                heap.push(Reverse(s));
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let sticks = vec![2,4,3];
        let result = Solution::connect_sticks(sticks);
        assert_eq!(result, 14);
    }

    #[test]
    fn t2() {
        let sticks = vec![1,8,3,5];
        let result = Solution::connect_sticks(sticks);
        assert_eq!(result, 30);
    }

    #[test]
    fn t3() {
        let sticks = vec![5];
        let result = Solution::connect_sticks(sticks);
        assert_eq!(result, 0);
    }

}
