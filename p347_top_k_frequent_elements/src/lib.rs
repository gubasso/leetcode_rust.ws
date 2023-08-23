struct Solution;
use std::{collections::{HashMap, BinaryHeap}, cmp::Reverse};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut counter = HashMap::new();
        let mut heap = BinaryHeap::new();
        for n in nums {
            // counter.entry(n).and_modify(|c|*c+=1).or_insert(1);
            *counter.entry(n).or_insert(0) += 1;
        }

        for (key,count) in counter {
            if heap.len() < k {
                heap.push(Reverse((count,key)));
            } else {
                let Reverse((min_count, _)) = heap.peek().unwrap();
                if count >= *min_count {
                    heap.push(Reverse((count,key)));
                    heap.pop(); // Some(Reverse(asld'[:]))
                }

            }
        }

        let mut result: Vec<i32> = heap.iter().map(|Reverse((_,key))| *key).collect();
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![1,1,1,2,2,3];
        let result = Solution::top_k_frequent(nums, 2);
        assert_eq!(result, vec![1,2]);
    }

    #[test]
    fn t2() {
        let nums = vec![1];
        let result = Solution::top_k_frequent(nums, 1);
        assert_eq!(result, vec![1]);
    }

}
