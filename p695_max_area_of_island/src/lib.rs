use std::collections::HashSet;

struct Solution;

type Land = (usize, usize);
type Grid = Vec<Vec<i32>>;
impl Solution {
    fn valid_coords(center: Land, m: usize, n: usize) -> [Option<Land>; 4] {
        let diffs: [i32; 5] = [0, 1, 0, -1, 0];
        diffs.windows(2).enumerate().fold([None; 4], |mut coords, (idx, d)| {
            let i = center.0 as i32 + d[0];
            let i = if i >= 0 && i < m as i32 {
                Some(i as usize)
            } else {
                None
            };

            let j = center.1 as i32 + d[1];
            let j = if j >= 0 && j < n as i32 {
                Some(j as usize)
            } else {
                None
            };

            if let (Some(i), Some(j)) = (i, j) {
                coords[idx] = Some((i, j));
            }

            coords
        })
    }

    fn dfs((i,j): Land, grid: &mut Grid, m: usize, n: usize) -> i32 {
        let mut size = 1;

        for (i,j) in Self::valid_coords((i,j), m, n).into_iter().flatten() {
            if grid[i][j] == 1 {
                grid[i][j] = 0;
                size += Self::dfs((i,j), grid, m, n);
            }
        }

        size
    }

    pub fn max_area_of_island(mut grid: Grid) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut max = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                    max = max.max(Self::dfs((i,j), &mut grid, m, n));
                }
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let grid = vec![vec![0,0,1,0,0,0,0,1,0,0,0,0,0],vec![0,0,0,0,0,0,0,1,1,1,0,0,0],vec![0,1,1,0,1,0,0,0,0,0,0,0,0],vec![0,1,0,0,1,1,0,0,1,0,1,0,0],vec![0,1,0,0,1,1,0,0,1,1,1,0,0],vec![0,0,0,0,0,0,0,0,0,0,1,0,0],vec![0,0,0,0,0,0,0,1,1,1,0,0,0],vec![0,0,0,0,0,0,0,1,1,0,0,0,0]];
        let result = Solution::max_area_of_island(grid);
        assert_eq!(result, 6);
    }

    #[test]
    fn t2() {
        let grid = vec![vec![0,0,0,0,0,0,0,0]];
        let result = Solution::max_area_of_island(grid);
        assert_eq!(result, 0);
    }

    #[test]
    fn t3() {
        let grid = vec![vec![1,1,0,0,0],vec![1,1,0,0,0],vec![0,0,0,1,1],vec![0,0,0,1,1]];
        let result = Solution::max_area_of_island(grid);
        assert_eq!(result, 4);
    }

}
