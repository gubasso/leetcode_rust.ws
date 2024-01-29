#[cfg(test)]
mod tests {
    struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let mut l = 0;
            let mut max_profit = 0;
            for i in 0..prices.len() {
                if prices[l] > prices[i] {
                    l = i;
                    continue;
                }
                max_profit = i32::max(max_profit, prices[i] - prices[l]);
            }
            max_profit
        }
    }

    #[test]
    fn t1() {
        let prices = vec![7,1,5,3,6,4];
        let output = 5;
        let result = Solution::max_profit(prices);
        assert_eq!(result, output);
    }

    #[test]
    fn t2() {
        let prices = vec![7,6,4,3,1];
        let output = 0;
        let result = Solution::max_profit(prices);
        assert_eq!(result, output);
    }

}
