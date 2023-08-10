struct Solution;
use std::collections::BinaryHeap;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);

        while let Some(s1) = heap.pop() {
            match heap.pop() {
                None => return s1,
                Some(s2) => {
                    let new = s1-s2;
                    if new > 0 {
                        heap.push(new);
                    }
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let stones = vec![2,7,4,1,8,1];
        let result = Solution::last_stone_weight(stones);
        assert_eq!(result, 1);
    }

    #[test]
    fn t2() {
        let stones = vec![1];
        let result = Solution::last_stone_weight(stones);
        assert_eq!(result, 1);
    }

}
