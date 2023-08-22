struct Solution;
use std::cmp::Ordering::{Equal,Less,Greater};
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut l, mut r) = (0, m*n);

        while l < r {
            let mid = l + (r - l)/2;
            match matrix[mid/n][mid%n].cmp(&target) {
                Equal => return true,
                Less => l = mid + 1,
                Greater => r = mid,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let matrix = vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]];
        let target = 3;
        let result = Solution::search_matrix(matrix,target);
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let matrix = vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]];
        let target = 13;
        let result = Solution::search_matrix(matrix,target);
        assert_eq!(result, false);
    }

}
