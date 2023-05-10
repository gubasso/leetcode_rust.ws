use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut nice: i32 = 0;
        let mut sum: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);

        for n in nums {
            sum += n % 2;
            if let Some(&val) = map.get(&(sum - k)) {
                nice += val;
            }
            // *map.entry(sum).or_insert(0) += 1;
            // same as:
            map.entry(sum).and_modify(|c| *c += 1).or_insert(1);
        }

        nice
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::number_of_subarrays(vec![1,1,2,1,1], 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let result = Solution::number_of_subarrays(vec![2,4,6], 1);
        assert_eq!(result, 0);
    }

    #[test]
    fn t3() {
        let result = Solution::number_of_subarrays(vec![2,2,2,1,2,2,1,2,2,2], 2);
        assert_eq!(result, 16);
    }

}
