struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut v = Vec::new();
        for i in 0..n {
            let pos = i + 1;
            let mut string: String = "".into();
            if pos % 3 == 0 { string.push_str("Fizz") };
            if pos % 5 == 0 { string.push_str("Buzz") };
            if !(pos % 3 == 0 || pos % 5 == 0) { string.push_str(&pos.to_string()) };
            v.push(string);
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::fizz_buzz(3);
        assert_eq!(result, vec!["1","2","Fizz"]);
    }

    #[test]
    fn t2() {
        let result = Solution::fizz_buzz(5);
        assert_eq!(result, vec!["1","2","Fizz","4","Buzz"]);
    }

    #[test]
    fn t3() {
        let result = Solution::fizz_buzz(15);
        assert_eq!(result, vec!["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]);
    }

}
