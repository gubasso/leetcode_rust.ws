struct Solution;
use std::{collections::HashMap, cmp::Reverse};
impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let half = (arr.len()/2) as i32;
        let mut map = HashMap::new();
        for n in arr {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut vec: Vec<(i32, i32)> = map.into_iter().collect();
        vec.sort_by_key(|(_,count)| Reverse(*count));
        let mut i = 0;
        let mut sum = 0;
        // println!("{vec:?}");

        while sum < half {
            sum += vec[i].1;
            i += 1;
            // println!("{sum:?}");
        }

        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let arr = vec![3,3,3,3,5,5,5,2,2,7];
        let result = Solution::min_set_size(arr);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let arr = vec![7,7,7,7,7,7];
        let result = Solution::min_set_size(arr);
        assert_eq!(result, 1);
    }

}
