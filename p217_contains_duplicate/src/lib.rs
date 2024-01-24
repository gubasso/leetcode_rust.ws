#[cfg(test)]
mod tests {
    struct Solution;

    use std::collections::HashSet;
    impl Solution {
        pub fn contains_duplicate(nums: Vec<i32>) -> bool {
            let mut set = HashSet::with_capacity(nums.len());
            for n in nums {
                if !set.insert(n) {
                    return true;
                }
            }
            false
        }
    }

    #[test]
    fn t1() {
        let result = Solution::contains_duplicate(vec![1,2,3,1]);
        assert!(result);
    }

    #[test]
    fn t2() {
        let result = Solution::contains_duplicate(vec![1,2,3,4]);
        assert!(!result);
    }

    #[test]
    fn t3() {
        let result = Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]);
        assert!(result);
    }

}
