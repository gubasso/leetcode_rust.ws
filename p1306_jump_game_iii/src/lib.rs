struct Solution;
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let (n, n_i) = (arr.len(), arr.len() as i32);
        let mut seen = vec![false; n];
        let su = start as usize;
        let mut stack = vec![su];
        seen[su] = true;

        while let Some(i) = stack.pop() {
            if arr[i] == 0 {
                return true;
            }
            let i_i = i as i32;
            let val = arr[i] as i32;
            for ni in [i_i+val,i_i-val].into_iter()
                .filter(|&v| v >= 0 && v < n_i)
                .map(|v| v as usize)
            {
                if !seen[ni] {
                    seen[ni] = true;
                    stack.push(ni);
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let arr = vec![4,2,3,0,3,1,2];
        let result = Solution::can_reach(arr, 5);
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let arr = vec![4,2,3,0,3,1,2];
        let result = Solution::can_reach(arr, 0);
        assert_eq!(result, true);
    }

    #[test]
    fn t3() {
        let arr = vec![3,0,2,1,2];
        let result = Solution::can_reach(arr, 2);
        assert_eq!(result, false);
    }

}
