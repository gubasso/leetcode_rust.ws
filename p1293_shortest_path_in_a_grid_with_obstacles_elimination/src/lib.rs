
struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m,n) = (grid.len(), grid[0].len());
        let (m_i,n_i) = (m as i32, n as i32);
        let mut seen = HashSet::from([((0,0),k)]);
        let mut queue = VecDeque::from([((0,0),k,0)]);

        while let Some(((i,j),remain,steps)) = queue.pop_front() {
            if (i,j) == (m_i-1,n_i-1) {
                return steps;
            }
            for (ni_i,nj_i) in [(i+1,j),(i-1,j),(i,j+1),(i,j-1)] {
                let (ni,nj) = (ni_i as usize, nj_i as usize);
                if ni < m && nj < n && seen.insert(((ni,nj),remain)) {
                    if grid[ni][nj] == 0 {
                        queue.push_back(((ni_i,nj_i),remain,steps+1));
                    }
                    if grid[ni][nj] == 1 && remain > 0 {
                        queue.push_back(((ni_i,nj_i),remain-1,steps+1));
                    }
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
        let grid = vec![vec![0,0,0],vec![1,1,0],vec![0,0,0],vec![0,1,1],vec![0,0,0]];
        let result = Solution::shortest_path(grid, 1);
        assert_eq!(result, 6);
    }

    #[test]
    fn t2() {
        let grid = vec![vec![0,1,1],vec![1,1,1],vec![1,0,0]];
        let result = Solution::shortest_path(grid, 1);
        assert_eq!(result, -1);
    }

    #[test]
    fn t3() {
        let grid = vec![vec![0,0],vec![1,0],vec![1,0],vec![1,0],vec![1,0],vec![1,0],vec![0,0],vec![0,1],vec![0,1],vec![0,1],vec![0,0],vec![1,0],vec![1,0],vec![0,0]];
        let result = Solution::shortest_path(grid, 4);
        assert_eq!(result, 14);
    }

}
