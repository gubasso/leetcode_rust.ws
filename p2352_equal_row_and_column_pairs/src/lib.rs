use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut row_map: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut col_vec: Vec<Vec<i32>> = Vec::new();

        for row_vec in grid.iter() {
            row_map.entry(row_vec.clone()).and_modify(|c| *c += 1 ).or_insert(1);
            for (j, col_n) in row_vec.iter().enumerate() {
                if let None = col_vec.get(j) {
                    col_vec.push(vec![*col_n]);
                } else {
                    col_vec[j].push(*col_n);
                }
            }
        }

        for col in &col_vec {
            if let Some(counter) = row_map.get(col) {
                ans += counter;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::equal_pairs(vec![vec![3,2,1],vec![1,7,6],vec![2,7,7]]);
        assert_eq!(result, 1);
    }

    #[test]
    fn t2() {
        let result = Solution::equal_pairs(vec![vec![3,1,2,2],vec![1,4,4,5],vec![2,4,2,2],vec![2,4,2,2]]);
        assert_eq!(result, 3);
    }

}
