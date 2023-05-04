use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let len = nums.len() as i32;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans = vec![];

        for arr in nums {
            for n in arr {
                map.entry(n).and_modify(|counter| *counter += 1).or_insert(1);
                if map.get(&n) == Some(&len) { ans.push(n); };
            }
        }

        ans.sort();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::intersection(vec![vec![3,1,2,4,5],vec![1,2,3,4],vec![3,4,5,6]]);
        assert_eq!(result, vec![3,4]);
    }

    #[test]
    fn t2() {
        let result = Solution::intersection(vec![vec![1,2,3],vec![4,5,6]]);
        assert_eq!(result, vec![]);
    }

}
