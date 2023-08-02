struct Solution;
use std::collections::{VecDeque};
impl Solution {
    fn get_valid_neighbours(i: i32, j: i32, m: i32, n: i32) -> Vec<(usize, usize)> {
        let mut coords = vec![];
        let is_valid = |i,j| {
            i >= 0 && j >= 0 && i < m && j < n
        };

        for di in -1..=1 {
            for dj in -1..=1 {
                if (di,dj) == (0,0) || (di != 0 && dj != 0) {
                    continue;
                }
                let r = i + di;
                let c = j + dj;
                if is_valid(r,c) {
                    coords.push((r as usize,c as usize));
                }
            }
        }

        coords
    }

    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m,n) = (mat.len(), mat[0].len());
        let mut queue = VecDeque::with_capacity(m*n);

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    queue.push_back((i,j));
                } else {
                    mat[i][j] = -1;
                }
            }
        }

        while let Some((i,j)) = queue.pop_front() {
            let (i_i, j_i) = (i as i32, j as i32);
            for (ni_i, nj_i) in [(i_i+1,j_i),(i_i-1,j_i),(i_i,j_i+1),(i_i,j_i-1)] {
                let (ni, nj) = (ni_i as usize, nj_i as usize);
                if ni < m && nj < n && mat[ni][nj] == -1 {
                    mat[ni][nj] = mat[i][j] + 1;
                    queue.push_back((ni, nj));
                }
            }
        }

        mat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Vec<i32>>>{
        vec![
            vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]],
            vec![vec![0,0,0],vec![0,1,0],vec![1,1,1]],
        ]
    }

    fn outputs() -> Vec<Vec<Vec<i32>>>{
        vec![
            vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]],
            vec![vec![0,0,0],vec![0,1,0],vec![1,2,1]],
        ]
    }

    #[test]
    fn t1() {
        let test_n = 1;
        let result = Solution::update_matrix(inputs()[test_n-1].clone());
        assert_eq!(result, outputs()[test_n-1]);
    }

    #[test]
    fn t2() {
        let test_n = 2;
        let result = Solution::update_matrix(inputs()[test_n-1].clone());
        assert_eq!(result, outputs()[test_n-1]);
    }

}
