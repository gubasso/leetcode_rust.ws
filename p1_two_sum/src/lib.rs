use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (i, curr) in nums.iter().enumerate() {
            let diff: i32 = target - curr;

            if let Some(idx) = map.get(&diff) {
                return vec![*idx, i as i32];
            } else {
                map.insert(*curr, i as i32);
            };

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
