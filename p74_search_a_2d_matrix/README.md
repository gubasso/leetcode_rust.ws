# p74_search_a_2d_matrix

Better implementation, with row and column derived from a index (working as if the matrix is a single array):

```rust
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
```

First implementation:

```rust
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
```
