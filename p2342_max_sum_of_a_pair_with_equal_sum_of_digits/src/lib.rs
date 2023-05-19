use std::collections::HashMap;

// fn to_digits(mut v: u64) -> Vec<u8> {
fn to_digits(mut v: i32) -> Vec<i32> {
    let mut digits: Vec<i32> = Vec::new();
    while v > 0 {
        let n = v % 10;
        v /= 10;
        digits.push(n);
    }
    digits
}

struct Solution;
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut vmap: Vec<i32> = vec![0; 82]; // (9*9)
        let mut ans = -1;
        for n in nums {
            let digits_sum = to_digits(n).iter().fold(0, |acc, x| acc + x);
            let key = digits_sum as usize;
            if vmap[key] > 0 {
                ans = i32::max(ans, vmap[key] + n);
            }
            vmap[key] = i32::max(vmap[key], n);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::maximum_sum(vec![18,43,36,13,7]);
        assert_eq!(result, 54);
    }

    #[test]
    fn t2() {
        let result = Solution::maximum_sum(vec![10,12,19,14]);
        assert_eq!(result, -1);
    }

    #[test]
    fn t3() {
        let result = Solution::maximum_sum(vec![279,169,463,252,94,455,423,315,288,64,494,337,409,283,283,477,248,8,89,166,188,186,128]);
        assert_eq!(result, 872);
    }

}
