struct Solution;
use std::{collections::{BinaryHeap, HashMap}, cmp::Reverse};

struct Median {
    map: HashMap<i32, i32>,
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl Median {

    fn new() -> Self {
        Median {
            map: HashMap::new(),
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn get_median(&self) -> f64 {
        let (nx, ni) = (self.max_heap.len(), self.min_heap.len());
        match (self.max_heap.peek(), self.min_heap.peek()) {
            (Some(&max), Some(&min)) if nx == ni => (max as f64 + min.0 as f64) / 2.0,
            (Some(&max), _) => max as f64,
            _ => 0.0,
        }
    }

    fn add_num(&mut self, num: i32) {
        self.map.entry(num).and_modify(|n|*n+=1).or_insert(1);
        self.max_heap.push(num);
        self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
        if self.min_heap.len() > self.max_heap.len() {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }

    fn pop(&mut self) -> Option<i32> {
        let (nx, ni) = (self.max_heap.len(), self.min_heap.len());
        if (nx, ni) == (0,0) {
            return None;
        }
        if nx == ni {
            return Some(self.min_heap.pop().unwrap().0);
        }
        if nx > ni {
            return self.max_heap.pop();
        }
        None
    }

    fn remove_num(&mut self, num: i32) -> Option<i32> {
        if !self.map.contains_key(&num) || self.map.get(&num).unwrap() == &0 {
            return None;
        }

        let mut bank_to_return: Vec<i32> = vec![];
        let mut removed_num: Option<i32> = None;

        while let Some(removed) = self.pop() {
            self.map.entry(removed).and_modify(|n|*n+=1);
            if removed == num {
                break;
            }
            bank_to_return.push(removed);
        }

        removed_num
    }

}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let (mut i, mut j, n) = (0, k as usize, nums.len());
        let mut ans = Vec::with_capacity(n);

        while j <= n {
            // let heap = Median::from_slice(&nums[i..j]);
            // ans.push(heap.get_median());
            // i += 1; j += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![1,3,-1,-3,5,3,6,7];
        let result = Solution::median_sliding_window(nums, 3);
        assert_eq!(result, vec![1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000]);
    }

    #[test]
    fn t2() {
        let nums = vec![1,2,3,4,2,3,1,4,2];
        let result = Solution::median_sliding_window(nums, 3);
        assert_eq!(result, vec![2.00000,3.00000,3.00000,3.00000,2.00000,3.00000,2.00000]);
    }

}
