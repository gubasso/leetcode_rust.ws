#[cfg(test)]
mod tests {
    struct Solution;
    use std::cmp::Ordering::{Equal, Less, Greater};
    impl Solution {
        pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
            let (m,n) = (matrix.len(), matrix[0].len());
            let (mut l, mut r) = (0, m*n-1);
            while l < r {
                let mid = l + (r - l + 1)/2;
                let number = matrix[mid/n][mid%n];
                match target.cmp(&number) {
                    Equal => return true,
                    Less => r = mid - 1,
                    Greater => l = mid,
                }
            }
            matrix[l/n][l%n] == target
        }
    }

    #[test]
    fn t1() {
        let matrix = vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]];
        let target = 3;
        let result = Solution::search_matrix(matrix, target);
        assert!(result);
    }

    #[test]
    fn t2() {
        let matrix = vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]];
        let target = 13;
        let result = Solution::search_matrix(matrix, target);
        assert!(!result);
    }

    #[test]
    fn t3() {
        let matrix = vec![vec![1]];
        let target = 1;
        let result = Solution::search_matrix(matrix, target);
        assert!(result);
    }

}
