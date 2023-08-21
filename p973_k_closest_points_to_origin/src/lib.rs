struct Solution;
use std::collections::BinaryHeap;
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        let d_2 = |point: &[i32]| (point[0]-0).pow(2) + (point[1]-0).pow(2);
        heap.push((d_2(&points[0]), points[0].clone()));

        for point in points.into_iter().skip(1) {
            let d = d_2(&point);
            if heap.len() < k {
                heap.push((d, point.clone()));
                continue;
            }
            if d < heap.peek().unwrap().0 {
                heap.push((d, point.clone()));
                heap.pop();
            }

        }

        heap.into_iter().map(|(_,p)| p).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let points = vec![vec![1,3],vec![-2,2]];
        let k = 1;
        let output = vec![vec![-2,2]];
        let result = Solution::k_closest(points, k);
        for r in result {
            assert!(output.contains(&r));
        }
    }

    #[test]
    fn t2() {
        let points = vec![vec![3,3],vec![5,-1],vec![-2,4]];
        let k = 2;
        let output = vec![vec![3,3],vec![-2,4]];
        let result = Solution::k_closest(points, k);
        for r in result {
            assert!(output.contains(&r));
        }
    }

}
