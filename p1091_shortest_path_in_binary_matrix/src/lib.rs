struct Solution;
use std::collections::{HashSet, VecDeque};
impl Solution {

    fn neighbours(row: i32, col: i32, m: i32, n: i32) -> Vec<(i32,i32)> {
        let mut all_coords = Vec::new();
        let is_valid = |(i,j): (i32, i32)| -> bool {
            i >= 0 && j >= 0 && i < m && j < n
        };

        for di in -1..=1 {
            for dj in -1..=1 {
                if (0,0) == (di,dj) {
                    continue;
                }
                let i = row + di;
                let j = col + dj;
                if  is_valid((i,j)) {
                    all_coords.push((i,j));
                }
            }
        }

        // for diff in [[0,1,0,-1,0], [1,1,-1,-1,1]] {
        //     for d in diff.windows(2) {
        //         let i = row + d[0];
        //         let j = col + d[1];
        //         if is_valid((i,j)) {
        //             all_coords.push((i,j));
        //         }
        //     }
        // }

        all_coords
    }

    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            return -1;
        }
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut seen = HashSet::from([(0,0)]);
        let mut queue = VecDeque::from([(0,0,1)]);

        while let Some((i, j, steps)) = queue.pop_front() {
            if (i, j) == (m-1, n-1) {
                return steps;
            }
            for (ni, nj) in Self::neighbours(i,j,m,n) {
                if grid[ni as usize][nj as usize] == 0 && seen.insert((ni,nj)) {
                    queue.push_back((ni, nj, steps + 1));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let grid = vec![vec![0,1],vec![1,0]];
        let result = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let grid = vec![vec![0,0,0],vec![1,1,0],vec![1,1,0]];
        let result = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(result, 4);
    }

    #[test]
    fn t3() {
        let grid = vec![vec![1,0,0],vec![1,1,0],vec![1,1,0]];
        let result = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(result, -1);
    }

    #[test]
    fn t4() {
        let grid = vec![vec![0,0,0],vec![1,1,0],vec![1,1,1]];
        let result = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(result, -1);
    }

}
