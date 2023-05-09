use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let (mut sum, mut ans) = (0, 0);
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);

        for n in nums {
            sum += n;
            if let Some(&val) = map.get(&(sum-k)) {
                ans += val;
            }
            *map.entry(sum).or_insert(0) += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::subarray_sum(vec![1,1,1], 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let result = Solution::subarray_sum(vec![1,2,3], 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn t3() {
        let result = Solution::subarray_sum(vec![1,-1,0], 0);
        assert_eq!(result, 3);
    }

}
