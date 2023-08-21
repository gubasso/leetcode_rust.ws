struct Solution;
use std::{collections::HashMap, cmp::Reverse};
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut map = HashMap::new();

        for &n in arr.iter() {
            *map.entry(n).or_insert(0) += 1;
        }

        let mut counts: Vec<i32> = map.into_iter().map(|(_,v)| v).collect();
        counts.sort_by_key(|n| Reverse(*n));

        while k > 0 {
            let n = counts.len();
            if counts[n-1] <= k {
                let c = counts.pop().unwrap();
                k -= c;
            } else {
                break;
            }
        }

        counts.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let arr = vec![5,5,4];
        let k = 1;
        let out = 1;
        let result = Solution::find_least_num_of_unique_ints(arr,k);
        assert_eq!(result, out);
    }

    #[test]
    fn t2() {
        let arr = vec![4,3,1,1,3,3,2];
        let k = 3;
        let out = 2;
        let result = Solution::find_least_num_of_unique_ints(arr,k);
        assert_eq!(result, out);
    }

}
