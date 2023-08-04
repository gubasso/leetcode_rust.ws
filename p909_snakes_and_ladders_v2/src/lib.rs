struct Solution;
use std::collections::VecDeque;
impl Solution {

    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let n2 = n.pow(2);
        let mut queue = VecDeque::from([0]);
        let mut dist = vec![-1; n2];
        dist[0] = 0;

        let boustrophedon = |val: usize| {
            let (i,j) = (val / n, val %n);
            match i%2 {
                0 => (n-1-i, j),    // even flip it
                _ => (n-1-i, n-1-j) // odd don't
            }
        };

        while let Some(node) = queue.pop_front() {

            for nd in (node+1)..(n2.min(node+7)) {
                let (ni, nj) = boustrophedon(nd);
                let dest = if board[ni][nj] != -1 {
                    (board[ni][nj]-1) as usize
                } else {
                    nd
                };
                if dist[dest] == -1 {
                    dist[dest] = dist[node] + 1;
                    queue.push_back(dest);
                }
            }
        }

        dist[n2-1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let board = vec![vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,35,-1,-1,13,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,15,-1,-1,-1,-1]];
        let result = Solution::snakes_and_ladders(board);
        assert_eq!(result, 4);
    }

    #[test]
    fn t2() {
        let board = vec![vec![-1,-1],vec![-1,3]];
        let result = Solution::snakes_and_ladders(board);
        assert_eq!(result, 1);
    }

    #[test]
    fn t3() {
        let board = vec![vec![-1,4,-1],vec![6,2,6],vec![-1,3,-1]];
        let result = Solution::snakes_and_ladders(board);
        assert_eq!(result, 2);
    }

    #[test]
    fn t4() {
        let board = vec![vec![-1,-1,-1],vec![-1,9,8],vec![-1,8,9]];
        let result = Solution::snakes_and_ladders(board);
        assert_eq!(result, 1);
    }

    #[test]
    fn t5() {
        let board = vec![vec![1,1,-1],vec![1,1,1],vec![-1,1,1]];
        let result = Solution::snakes_and_ladders(board);
        assert_eq!(result, -1);
    }

}
