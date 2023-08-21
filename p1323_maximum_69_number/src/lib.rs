struct Solution;
impl Solution {
    pub fn maximum69_number (mut num: i32) -> i32 {
        for p in (1..5).rev() {
            let x = (num % 10_i32.pow(p)) / (10_i32.pow(p-1));
            if x == 6 {
                // num -= x*10_i32.pow(p-1);
                // num += 9*10_i32.pow(p-1);
                // return num;
                // println!("{num}");
                return num + 10_i32.pow(p-1)*3;
            }
            // println!("{}", x*10_i32.pow(p-1));
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let num = 9669;
        let result = Solution::maximum69_number(num);
        assert_eq!(result, 9969);
    }

    #[test]
    fn t2() {
        let num = 9996;
        let result = Solution::maximum69_number(num);
        assert_eq!(result, 9999);
    }

    #[test]
    fn t3() {
        let num = 9999;
        let result = Solution::maximum69_number(num);
        assert_eq!(result, 9999);
    }

}
