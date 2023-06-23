// https://leetcode.com/problems/daily-temperatures/solutions/2924618/simple-rust-o-n-solution-using-monotonic-stack/

struct Solution;
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let temp = temperatures;
        let mut ans = vec![0; n];
        let mut stack: Vec<usize> = vec![];

        for i in 0..n {
            while stack.last().map_or(false, |&j| temp[i] > temp[j]) {
                stack.pop().map(|j| {
                    ans[j] = (i - j) as i32;
                });
            };
            stack.push(i);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]);
        assert_eq!(result, vec![1,1,4,2,1,1,0,0]);
    }

    #[test]
    fn t2() {
        let result = Solution::daily_temperatures(vec![30,40,50,60]);
        assert_eq!(result, vec![1,1,1,0]);
    }

    #[test]
    fn t3() {
        let result = Solution::daily_temperatures(vec![30,60,90]);
        assert_eq!(result, vec![1,1,0]);
    }

    #[test]
    fn t4() {
        let result = Solution::daily_temperatures(vec![89,62,70,58,47,47,46,76,100,70]);
        assert_eq!(result, vec![8,1,5,4,3,2,1,1,0,0]);
    }

}
