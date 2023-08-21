struct Solution;
impl Solution {
    pub fn max_number_of_apples(mut weight: Vec<i32>) -> i32 {
        let mut remainder = 5000;
        weight.sort();
        let mut i = 0;

        while i < weight.len() {
            remainder -= weight[i];
            if remainder < 0 {
                return i as i32;
            }
            i += 1;
        }

        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let weight = vec![100,200,150,1000];
        let result = Solution::max_number_of_apples(weight);
        assert_eq!(result, 4);
    }

    #[test]
    fn t2() {
        let weight = vec![900,950,800,1000,700,800];
        let result = Solution::max_number_of_apples(weight);
        assert_eq!(result, 5);
    }

    #[test]
    fn t3() {
        let weight = vec![1000,1000,1000,1000,1000];
        let result = Solution::max_number_of_apples(weight);
        assert_eq!(result, 5);
    }

}
