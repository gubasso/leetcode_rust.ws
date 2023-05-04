use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        let mut counter: i32 = 0;
        let set: HashSet<i32> = arr.clone().into_iter().collect();

        for n in arr {
            if set.contains(&(n + 1)) {
                counter += 1;
            }
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::count_elements(vec![1,2,3]);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let result = Solution::count_elements(vec![1,1,3,3,5,5,7,7]);
        assert_eq!(result, 0);
    }

    #[test]
    fn t3() {
        let result = Solution::count_elements(vec![1,3,2,3,5,0]);
        assert_eq!(result, 3);
    }

    #[test]
    fn t4() {
        let result = Solution::count_elements(vec![1,1,2,2]);
        assert_eq!(result, 2);
    }

}
