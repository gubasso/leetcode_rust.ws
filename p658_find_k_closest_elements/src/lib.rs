struct Solution;
use std::collections::BinaryHeap;
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        heap.push(((arr[0] - x).abs(), arr[0]));

        for n in arr.into_iter().skip(1) {
            let dist = (n-x).abs();
            if heap.len() < k {
                heap.push((dist, n));
                continue;
            }
            let (diff, num) = *heap.peek().unwrap();
            if dist < diff || (dist == diff && n <= num) {
                heap.pop();
                heap.push((dist, n));
            }
        }

        let mut ans: Vec<i32> = heap.into_iter().map(|(_,num)| num).collect();
        ans.sort();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let arr = vec![1,2,3,4,5];
        let result = Solution::find_closest_elements(arr, 4, 3);
        assert_eq!(result, vec![1,2,3,4]);
    }

    #[test]
    fn t2() {
        let arr = vec![1,2,3,4,5];
        let result = Solution::find_closest_elements(arr, 4, -1);
        assert_eq!(result, vec![1,2,3,4]);
    }

}
