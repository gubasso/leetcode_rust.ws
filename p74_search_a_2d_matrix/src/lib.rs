struct Solution;
use std::cmp::Ordering::{Equal,Less,Greater};
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        if target < matrix[0][0] || target > matrix[m-1][n-1] {
            return false;
        }
        let bs = |arr: &[i32], t: i32| -> usize {
            let (mut l, mut r) = (0, arr.len());
            while l < r {
                let mid = l + (r-l)/2;
                match arr[mid].cmp(&t) {
                    Equal => return mid,
                    Less => l = mid+1,
                    Greater => r = mid,
                }
            }
            l-1
        };
        let first_col: Vec<i32> = matrix.iter().map(|v| v[0]).collect();
        let i = bs(&first_col, target);
        let j = bs(&matrix[i], target);
        if target == matrix[i][j] {
            return true;
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
