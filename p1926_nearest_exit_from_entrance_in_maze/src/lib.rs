struct Solution;
use std::collections::{VecDeque, HashSet};
impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let (m,n) = (maze.len(), maze[0].len());
        let entry = (entrance[0], entrance[1]);
        let mut queue = VecDeque::from([entry]);
        maze[entry.0 as usize][entry.1 as usize] = '+';
        let mut level = 0;

        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                let (i,j) = queue.pop_front().unwrap();
                for (ni_i,nj_i) in [(i+1,j),(i-1,j),(i,j+1),(i,j-1)] {
                    let (ni,nj) = (ni_i as usize, nj_i as usize);
                    if ni < m && nj < n && maze[ni][nj] != '+' {
                        if ni == 0 || nj == 0 || ni == m-1 || nj == n-1 {
                            return level+1;
                        }
                        maze[ni][nj] = '+';
                        queue.push_back((ni_i, nj_i));
                    }
                }


            }
            level += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let maze = vec![vec!['+','+','.','+'],vec!['.','.','.','+'],vec!['+','+','+','.']];
        let entrance = vec![1,2];
        let result = Solution::nearest_exit(maze, entrance);
        assert_eq!(result, 1);
    }

    #[test]
    fn t2() {
        let maze = vec![vec!['+','+','+'],vec!['.','.','.'],vec!['+','+','+']];
        let entrance = vec![1,0];
        let result = Solution::nearest_exit(maze, entrance);
        assert_eq!(result, 2);
    }

    #[test]
    fn t3() {
        let maze = vec![vec!['.','+']];
        let entrance = vec![0,0];
        let result = Solution::nearest_exit(maze, entrance);
        assert_eq!(result, -1);
    }

    #[test]
    fn t4() {
        let maze = vec![vec!['.','+','+','+','.','.','.','+','+'],vec!['.','.','+','.','+','.','+','+','.'],vec!['.','.','+','.','.','.','.','.','.'],vec!['.','+','.','.','+','+','.','+','.'],vec!['.','.','.','.','.','.','.','+','.'],vec!['.','.','.','.','.','.','.','.','.'],vec!['.','.','.','+','.','.','.','.','.'],vec!['.','.','.','.','.','.','.','.','+'],vec!['+','.','.','.','+','.','.','.','.']];
        let entrance = vec![5,6];
        let result = Solution::nearest_exit(maze, entrance);
        assert_eq!(result, 2);
    }

}
