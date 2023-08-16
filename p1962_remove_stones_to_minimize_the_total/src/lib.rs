struct Solution;
use std::collections::BinaryHeap;
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        // println!("\n");
        let mut heap = BinaryHeap::from(piles);
        for _ in 0..k {
            let stones = heap.pop().unwrap();
            let n = stones - (stones/2);
            // println!("{:?}", n);
            heap.push(n);
        }
        heap.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let piles = vec![5,4,9];
        let result = Solution::min_stone_sum(piles, 2);
        assert_eq!(result, 12);
    }

    #[test]
    fn t2() {
        let piles = vec![4,3,6,7];
        let result = Solution::min_stone_sum(piles, 3);
        assert_eq!(result, 12);
    }

}
