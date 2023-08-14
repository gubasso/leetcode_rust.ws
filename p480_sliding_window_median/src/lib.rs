struct Solution;
use std::{collections::{BinaryHeap, HashMap, BTreeMap}, cmp::Reverse};

#[derive(Default, Debug)]
struct MultiSet<T> {
    count: usize,
    data: BTreeMap<T, usize>,
}

impl<T: Ord + Copy> MultiSet<T> {

    fn insert(&mut self, val: T) {
        self.data.entry(val)
            .and_modify(|c| *c+=1)
            .or_insert(1);
        self.count += 1;
    }

    fn remove(&mut self, val: &T) -> Option<T> {
        if let Some(count) = self.data.get_mut(val) {
            *count -= 1;
            if *count == 0 {
                self.data.remove(val);
            }
            self.count -= 1;
            return Some(*val);
        }
        None
    }

    fn last(&mut self) -> Option<T> {
        self.data.iter().next_back().map(|(k, _)| *k)
        // self.data.last_key_value().map(|(k,_)| *k)
    }

    fn pop_last(&mut self) -> Option<T> {
        self.last().and_then(|l| self.remove(&l))
    }

    fn len(&self) -> usize {
        self.count
    }

}

#[derive(Default, Debug)]
struct MedianFinder {
    lo: MultiSet<i64>,
    hi: MultiSet<Reverse<i64>>,
}

impl MedianFinder {

    fn get_median(&mut self) -> Option<f64> {
        let max = self.lo.last()?;
        if self.lo.len() != self.hi.len() {
            return Some(max as f64);
        }
        let min = self.hi.last().unwrap().0;
        Some((max + min) as f64 / 2.0)
    }

    fn push(&mut self, val: i64) {
        self.lo.insert(val);
        self.hi.insert(Reverse(self.lo.pop_last().unwrap()));
        // println!("hi: {:?}", self.hi);
        if self.lo.len() < self.hi.len() {
            self.lo.insert(self.hi.pop_last().unwrap().0);
        }
    }

    fn pop(&mut self, val: i64) -> Option<i64> {
        if let Some(popped) = self.lo.remove(&val) {
            if self.lo.len() < self.hi.len() {
                self.lo.insert(self.hi.pop_last().unwrap().0);
            }
            return Some(popped);
        }
        if let Some(popped) = self.hi.remove(&Reverse(val)) {
            if self.lo.len() - self.hi.len() > 1  {
                self.hi.insert(Reverse(self.lo.pop_last().unwrap()));
            }
            return Some(popped.0);
        }
        None
    }

    fn balance(&mut self) {
        while self.lo.len() < self.hi.len() {
            self.lo.insert(self.hi.pop_last().unwrap().0);
        }
        while self.lo.len() - self.hi.len() > 1  {
            self.hi.insert(Reverse(self.lo.pop_last().unwrap()));
        }
    }

}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        // println!("\n");
        let (n, k) = (nums.len(), k as usize);
        let nums = nums.into_iter().map(|e| e as i64).collect::<Vec<i64>>();
        let mut ans = Vec::with_capacity(n);
        let mut mf = MedianFinder::default();

        // for l in 0..k {
        for n in nums.iter().take(k) {
            mf.push(*n);
        }
        ans.push(mf.get_median().unwrap());

        for i in k..nums.len() {
            // println!("to add: {:?}, to remove: {:?}", nums[i], nums[i-k]);
            mf.push(nums[i]);
            mf.pop(nums[i-k]);
            // println!("lo: {:?}", mf.lo);
            // println!("hi: {:?}", mf.hi);
            // println!("- median: {:?}", mf.get_median().unwrap());
            ans.push(mf.get_median().unwrap());
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn utils() {
    //     println!("\n");
    //     let nums = vec![1,3,-1,-3,5,3,6,7];
    //     let mut median = Median::new();
    //     for n in nums {
    //         median.add_num(n);
    //     }
    //     println!("{:?}", median);
    //     median.remove_num(-1);
    //     println!("{:?}", median);
    //     median.remove_num(3);
    //     println!("{:?}", median);
    //     median.remove_num(3);
    //     println!("{:?}", median);
    // }

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

    #[test]
    fn t3() {
        let nums = vec![7,9,3,8,0,2,4,8,3,9];
        let result = Solution::median_sliding_window(nums, 1);
        assert_eq!(result, vec![7.00000,9.00000,3.00000,8.00000,0.00000,2.00000,4.00000,8.00000,3.00000,9.00000]);
    }

    #[test]
    fn t4() {
        let nums = vec![6,5,9,5,4,9,1,7,5,5];
        let result = Solution::median_sliding_window(nums, 4);
        assert_eq!(result, vec![5.50000,5.00000,7.00000,4.50000,5.50000,6.00000,5.00000]);
    }

}
