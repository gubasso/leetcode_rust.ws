struct Solution;
use std::collections::BinaryHeap;
impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects: Vec<(i32, i32)> = capital.into_iter().zip(profits.into_iter()).collect();
        projects.sort();
        let mut heap = BinaryHeap::new();
        let mut i = 0;

        for _ in 0..k {
            while i < projects.len() && projects[i].0 <= w {
                heap.push(projects[i].1);
                i += 1;
            }
            if heap.is_empty() {
                return w;
            }
            w += heap.pop().unwrap();
        }

        w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let k = 2;
        let w = 0;
        let profits = vec![1,2,3];
        let capital = vec![0,1,1];
        let result = Solution::find_maximized_capital(k,w,profits,capital);
        assert_eq!(result, 4);
    }

    #[test]
    fn t2() {
        let k = 3;
        let w = 0;
        let profits = vec![1,2,3];
        let capital = vec![0,1,2];
        let result = Solution::find_maximized_capital(k,w,profits,capital);
        assert_eq!(result, 6);
    }

}
