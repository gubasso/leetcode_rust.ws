use std::{collections::BinaryHeap, cmp::Reverse};

#[derive(Debug)]
struct KthLargest {
    h: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut obj = KthLargest {
            h: BinaryHeap::new(),
            k: k as usize,
        };
        for &n in nums.iter() {
            obj.add(n);
        }
        // println!("{:?}", obj);
        obj
    }

    fn add(&mut self, val: i32) -> i32 {
        if let Some(&Reverse(curr)) = self.h.peek() {
            if self.h.len() == self.k && val <= curr {
                return curr;
            }
        }

        self.h.push(Reverse(val));

        if self.h.len() > self.k {
            self.h.pop();
        }
        // println!("heap: {:?}", self.h);
        self.h.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        // println!("\n");
        let k = 3;
        let nums = vec![4, 5, 8, 2];
        let mut obj = KthLargest::new(k, nums);
        let input = [3, 5, 10, 9, 4];
        let output = [4, 5, 5, 8, 8];
        for i in 0..input.len() {
            let ret = obj.add(input[i]);
            assert_eq!(ret, output[i]);
        }
    }

    #[test]
    fn t2() {
        // println!("\n");
        let k = 1;
        let nums = vec![];
        let mut obj = KthLargest::new(k, nums);
        let input = [-3,-2,-4,0,4];
        let output = [-3,-2,-2,0,4];
        for i in 0..input.len() {
            let ret = obj.add(input[i]);
            assert_eq!(ret, output[i]);
        }
    }

}
