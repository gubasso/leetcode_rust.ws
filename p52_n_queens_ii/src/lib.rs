struct Solution;
use std::collections::HashSet;
impl Solution {

    fn backtrack(n: i32, i: i32,
                 cols: &mut HashSet<i32>,
                 diags: &mut HashSet<i32>,
                 anti_diags: &mut HashSet<i32>) -> i32 {

        if n == i {
            return 1;
        }

        let mut solutions = 0;

        for j in 0..n {
            let curr_diag = i - j;
            let curr_anti_diag = i + j;
            if cols.contains(&j) ||
            diags.contains(&curr_diag) ||
            anti_diags.contains(&curr_anti_diag) {
                continue;
            }

            cols.insert(j);
            diags.insert(curr_diag);
            anti_diags.insert(curr_anti_diag);
            solutions += Self::backtrack(n, i+1, cols, diags, anti_diags);
            cols.remove(&j);
            diags.remove(&curr_diag);
            anti_diags.remove(&curr_anti_diag);
        }

        solutions
    }

    pub fn total_n_queens(n: i32) -> i32 {
        let mut cols: HashSet<i32> = HashSet::new();
        let mut diags: HashSet<i32> = HashSet::new();
        let mut anti_diags: HashSet<i32> = HashSet::new();
        Self::backtrack(n, 0, &mut cols, &mut diags, &mut anti_diags)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::total_n_queens(4);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let result = Solution::total_n_queens(1);
        assert_eq!(result, 1);
    }

}
