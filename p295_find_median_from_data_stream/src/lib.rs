use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug)]
struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {

    fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        if let Some(max) = self.max_heap.pop() {
            self.min_heap.push(Reverse(max));
        }
        if self.max_heap.len() < self.min_heap.len() {
            if let Some(min) = self.min_heap.pop() {
                self.max_heap.push(min.0);
            }
        }
    }

    fn find_median(&self) -> f64 {
        let (nx, ni) = (self.max_heap.len(), self.min_heap.len());
        match (self.max_heap.peek(), self.min_heap.peek()) {
            (Some(&max), Some(&min)) if nx == ni => (max as f64 + min.0 as f64)/2.0,
            (Some(&max), _) => max as f64,
            _ => 0.0,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        println!("\n");
        let mut obj = MedianFinder::new();
        obj.add_num(1);
        obj.add_num(2);
        println!("{:?}", obj);
        assert_eq!(obj.find_median(), 1.5);
        obj.add_num(3);
        println!("{:?}", obj);
        assert_eq!(obj.find_median(), 2.0);
    }

}
