// use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut lost_count: Vec<i32> = vec![-1; 100001];
        let mut ans = vec![vec![], vec![]];

        for m in matches {
            let winner = m[0] as usize;
            let loser = m[1] as usize;
            lost_count[winner] += if lost_count[winner] == -1 { 1 } else { 0 };
            lost_count[loser] += if lost_count[loser] == -1 { 2 } else { 1 };
        }

        for (player, losses) in lost_count.iter().enumerate() {
            if *losses == 0 { ans[0].push(player as i32) };
            if *losses == 1 { ans[1].push(player as i32) };
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let matches = vec![vec![1,3],vec![2,3],vec![3,6],vec![5,6],vec![5,7],vec![4,5],vec![4,8],vec![4,9],vec![10,4],vec![10,9]];
        let result = Solution::find_winners(matches);
        assert_eq!(result, vec![vec![1,2,10],vec![4,5,7,8]]);
    }

    #[test]
    fn t2() {
        let matches = vec![vec![2,3],vec![1,3],vec![5,4],vec![6,4]];
        let result = Solution::find_winners(matches);
        assert_eq!(result, vec![vec![1,2,5,6],vec![]]);
    }

}
