struct Solution;
impl Solution {

    fn backtrack(ans: &mut Vec<Vec<i32>>, tup: (i32, i32), mut curr: Vec<i32>, i: i32) {
        let (n,k) = tup;
        if curr.len() == k as usize {
            ans.push(curr.to_vec());
            return;
        }

        for num in i..=n {
            curr.push(num);
            Self::backtrack(ans, tup, curr.to_vec(), num+1);
            curr.pop();
        }

    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        Self::backtrack(&mut ans, (n,k), vec![], 1);
        ans
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let (n, k) = (4, 2);
        let out = vec![vec![1,2],vec![1,3],vec![1,4],vec![2,3],vec![2,4],vec![3,4]];
        let result = Solution::combine(n,k);
        assert_eq!(result, out);
    }

    #[test]
    fn t2() {
        let (n, k) = (1, 1);
        let out = vec![vec![1]];
        let result = Solution::combine(n,k);
        assert_eq!(result, out);
    }

}
