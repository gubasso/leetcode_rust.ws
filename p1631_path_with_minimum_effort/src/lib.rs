struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        // println!("\n");
        let (m,n) = (heights.len(), heights[0].len());
        if (m,n) == (1,1) {
            return 0;
        }
        let directions = [(0,1),(0,-1),(1,0),(-1,0)];
        let is_valid = |i: i32, j: i32| -> bool {
            i >= 0 && j >= 0 && i < m as i32 && j < n as i32
        };
        let get_valid_neighbours = |(i,j): (usize, usize), effort: i32| -> Vec<(usize, usize)> {
            let mut neighbours = vec![];
            for &(di, dj) in directions.iter() {
                let (ni, nj) = (di+i as i32, dj+j as i32);
                if is_valid(ni, nj) {
                    let (ni, nj) = (ni as usize, nj as usize);
                    if (heights[i][j]-heights[ni][nj]).abs() <= effort{
                        neighbours.push((ni, nj));
                    }
                }
            }
            neighbours
        };
        let check = |effort: i32| -> bool {
            let mut stack = vec![(0,0)];
            let mut seen = HashSet::from([(0,0)]);

            while let Some((i,j)) = stack.pop() {
                for neighbour in get_valid_neighbours((i,j), effort) {
                    if seen.insert(neighbour) {
                        // println!("{:?}", neighbour);
                        if neighbour == (m-1, n-1) {
                            return true;
                        }
                        stack.push(neighbour);
                    }
                }
            }
            false
        };
        let (mut l, mut r) = (0, heights.iter().flatten().max().unwrap()+1);

        while l < r {
            let effort = l + (r-l)/2;
            match check(effort) {
                false => l=effort+1,
                true => r=effort
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let heights = vec![vec![1,2,2],vec![3,8,2],vec![5,3,5]];
        let result = Solution::minimum_effort_path(heights);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let heights = vec![vec![1,2,3],vec![3,8,4],vec![5,3,5]];
        let result = Solution::minimum_effort_path(heights);
        assert_eq!(result, 1);
    }

    #[test]
    fn t3() {
        let heights = vec![vec![1,2,1,1,1],vec![1,2,1,2,1],vec![1,2,1,2,1],vec![1,2,1,2,1],vec![1,1,1,2,1]];
        let result = Solution::minimum_effort_path(heights);
        assert_eq!(result, 0);
    }

    #[test]
    fn t4() {
        let heights = vec![vec![3]];
        let result = Solution::minimum_effort_path(heights);
        assert_eq!(result, 0);
    }

}
