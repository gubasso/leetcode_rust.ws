use std::collections::{HashMap, VecDeque};

struct Solution;
impl Solution {
    fn bfs(start: usize, is_connected: &Vec<Vec<i32>>, seen: &mut Vec<bool>) {
        let len = is_connected.len();
        let mut queue = VecDeque::with_capacity(len);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            for j in 0..len {
                if !seen[j] && is_connected[node][j] > 0 {
                    queue.push_back(j);
                    seen[j] = true;
                }
            }
        }

    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let len = is_connected.len();
        let mut groups = 0;
        let mut seen = vec![false; len];

        for i in 0..len {
            if seen[i] { continue; }
            groups += 1;
            seen[i] = true;
            Self::bfs(i, &is_connected, &mut seen);
        }

        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let input = vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]];
        let result = Solution::find_circle_num(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let input = vec![vec![1,0,0],vec![0,1,0],vec![0,0,1]];
        let result = Solution::find_circle_num(input);
        assert_eq!(result, 3);
    }

}
