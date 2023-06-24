use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut dec = Vec::new();
        let mut map = HashMap::new();

        for n in nums2 {
            while dec.last().map_or(false, |&m| n > m) {
                map.insert(dec.pop().unwrap(), n);
            }
            dec.push(n);
        }

        for n in nums1 {
            // let val = map.get(&n).map_or(-1, |&v| v);
            let val = *map.get(&n).unwrap_or(&-1);
            ans.push(val);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::next_greater_element(vec![4,1,2], vec![1,3,4,2]);
        assert_eq!(result, vec![-1,3,-1]);
    }

    #[test]
    fn t2() {
        let result = Solution::next_greater_element(vec![2,4], vec![1,2,3,4]);
        assert_eq!(result, vec![3,-1]);
    }

}

