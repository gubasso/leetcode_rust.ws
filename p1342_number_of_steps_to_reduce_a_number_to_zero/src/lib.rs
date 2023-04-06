struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut steps: i32 = 0;
        let mut n = num;

        while n != 0 {
            if n % 2 == 0 {
                n = n / 2;
            } else {
                n -= 1;
            }
            steps += 1;
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::number_of_steps(14);
        assert_eq!(result, 6);
    }

    #[test]
    fn t2() {
        let result = Solution::number_of_steps(8);
        assert_eq!(result, 4);
    }

    #[test]
    fn t3() {
        let result = Solution::number_of_steps(123);
        assert_eq!(result, 12);
    }

}
