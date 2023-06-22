use std::collections::VecDeque;

struct MovingAverage {
    size: i32,
    values: VecDeque<i32>,
    sum: i32,
}

impl MovingAverage {

    fn new(size: i32) -> Self {
        MovingAverage {
            size,
            values: VecDeque::new(),
            sum: 0,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.values.push_back(val);
        let mut n = self.values.len() as i32;
        if n > self.size {
            self.sum -= self.values.pop_front().unwrap();
            n = self.size;
        }
        self.sum += val;
        ((self.sum as f64 / n as f64) * 100000.0).round() / 100000.0
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let mut obj = MovingAverage::new(3);
        let values = vec![vec![1], vec![10], vec![3], vec![5]];
        let answers = vec![1.0, 5.5, 4.66667, 6.0];
        for i in 0..values.len() {
            let average = obj.next(*values[i].first().unwrap());
            assert_eq!(average, answers[i]);
        }
    }

}
