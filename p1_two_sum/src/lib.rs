use std::collections::HashMap;

// Hashmap solution:
// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut map: HashMap<i32, i32> = HashMap::new();
//
//     for (i, curr) in nums.iter().enumerate() {
//         match map.get(&(target - curr)) {
//             Some(&idx) => return vec![idx, i as i32],
//             None => map.insert(*curr, i as i32),
//         };
//     }
//     vec![]
// }

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left: usize = 0;
        let mut right: usize = nums.len()-1;
        let mut ordered: Vec<(usize, &i32)> = nums.iter().enumerate().collect();
        ordered.sort_by(|a,b| a.1.cmp(&b.1));

        for _e in &ordered {
            let sum = ordered[left].1 + ordered[right].1;
            if sum == target {
                return vec![ordered[left].0 as i32, ordered[right].0 as i32];
            } else if sum < target {
                left += 1;
            } else if sum > target {
                right -= 1;
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::two_sum(vec![2,7,11,15], 9);
        assert_eq!(result, vec![0,1]);
    }

    #[test]
    fn t2() {
        let result = Solution::two_sum(vec![3,2,4], 6);
        assert_eq!(result, vec![1,2]);
    }

    #[test]
    fn t3() {
        let result = Solution::two_sum(vec![3,3], 6);
        assert_eq!(result, vec![0,1]);
    }

}
