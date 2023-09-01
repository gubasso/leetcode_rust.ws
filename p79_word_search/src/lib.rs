struct Solution;

use std::collections::HashSet;
impl Solution {

    fn neighbours(m: usize, n: usize, i: i32, j: i32) -> Vec<(usize, usize)> {
        [(0,1),(0,-1),(1,0),(-1,0)].into_iter().fold(vec![], |mut v, d| {
            let (ni,nj) = (i+d.0, j+d.1);
            if ni >= 0 && nj >= 0 && ni < m as i32 && nj < n as i32 {
                v.push((ni as usize, nj as usize));
            }
            v
        })
    }

    fn backtrack(board: &Vec<Vec<char>>, boundaries: (usize,usize),
                 word: &Vec<char>, seen: &mut HashSet<(usize,usize)>,
                 li: usize, i: usize, j: usize) -> bool {
        if li == word.len() {
            return true;
        }

        // println!("word idx: {li}, letter: {}", word[li]);
        // println!("board letter: ({i},{j}), letter: {}", board[i][j]);

        let (m,n) = boundaries;

        for (ni,nj) in Self::neighbours(m, n, i as i32, j as i32) {
            // println!("neighbour ({ni}, {nj})");
            // println!("neighbour ({ni}, {nj}), letter: {}", board[ni][nj]);
            if board[ni][nj] == word[li] && seen.insert((ni,nj)) {
                if Self::backtrack(board, boundaries, word, seen, li+1, ni, nj) {
                    return true;
                }
                seen.remove(&(ni,nj));
            }
        }

        false
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        // println!("\n");

        let boundaries = (board.len(), board[0].len());
        let (m,n) = boundaries;
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == word[0] {
                    // println!("## start: ({i},{j}), {} = {}", board[i][j], word[0]);
                    let mut seen: HashSet<(usize, usize)> = HashSet::from([(i,j)]);
                    if Self::backtrack(&board, boundaries, &word, &mut seen, 1, i, j) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        let word = "ABCCED".to_string();
        let result = Solution::exist(board,word);
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        let word = "SEE".to_string();
        let result = Solution::exist(board,word);
        assert_eq!(result, true);
    }

    #[test]
    fn t3() {
        let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        let word = "ABCB".to_string();
        let result = Solution::exist(board,word);
        assert_eq!(result, false);
    }

}
