struct Solution;

use std::collections::VecDeque;
type Bomb = (i64, i64, u64);
impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {

        // println!("\n");
        let n = bombs.len();
        let mut graph = vec![vec![]; n];
        let mut max_exploded = 1;

        for from in 0..n {
            for to in 0..n {
                if from == to {
                    continue;
                }
                let (xf,yf,rf) = (bombs[from][0] as i64,
                                  bombs[from][1] as i64,
                                  bombs[from][2] as u64);
                let (xt,yt,_) = (bombs[to][0] as i64,
                                  bombs[to][1] as i64,
                                  bombs[to][2] as u64);
                let lhs = rf.pow(2);
                let rhs = ((xt-xf).pow(2) + (yt-yf).pow(2)) as u64;
                if lhs >= rhs {
                    graph[from].push(to);
                }
            }
        }

        // println!("{graph:?}");

        for first_bomb in 0..n {
            let mut queue = VecDeque::from([first_bomb]);
            let mut seen = vec![false; n];
            seen[first_bomb] = true;
            let mut total_explosions = 0;
            // println!("# FIRST BOMB = {}", first_bomb);

            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    let bomb = queue.pop_front().unwrap();
                    total_explosions += 1;
                    // println!("bomb: {}", bomb);
                    for &exploded in graph[bomb].iter() {
                        if !seen[exploded] {
                            // println!("exploded -> {}", exploded);
                            seen[exploded] = true;
                            queue.push_back(exploded);
                        }
                    }
                }
                // println!("partial explosions: {}", total_explosions);

            }

            // println!("first bomb: {} total explosions: {}", first_bomb, total_explosions);

            max_exploded = max_exploded.max(total_explosions);
        }

        max_exploded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let bombs = vec![vec![2,1,3],vec![6,1,4]];
        let result = Solution::maximum_detonation(bombs);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let bombs = vec![vec![1,1,5],vec![10,10,5]];
        let result = Solution::maximum_detonation(bombs);
        assert_eq!(result, 1);
    }

    #[test]
    fn t3() {
        let bombs = vec![vec![1,2,3],vec![2,3,1],vec![3,4,2],vec![4,5,3],vec![5,6,4]];
        let result = Solution::maximum_detonation(bombs);
        assert_eq!(result, 5);
    }

    #[test]
    fn t4() {
        let bombs = vec![vec![1,1,100000],vec![100000,100000,1]];
        let result = Solution::maximum_detonation(bombs);
        assert_eq!(result, 1);
    }

}
