use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 0);

        for n in nums {
            sum += n;
            let wished_sum = sum - k;
            let map_wished_sum_val = map.get(&wished_sum).cloned();

            if let Some(_) = map_wished_sum_val {
                let mut map_to_add = *map.get_mut(&wished_sum).unwrap();
                map_to_add += 1;
                ans += map_to_add;
                map.entry(sum).and_modify(|val| *val += 1).or_insert(0);
            } else {
                map.entry(sum).and_modify(|val| *val += 1).or_insert(0);
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
