use std::cmp;

struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max_wealth: i32 = 0;
        for client in accounts {
            let mut client_wealth: i32 = 0;
            for acc_wealth in client {
                client_wealth += acc_wealth;
            }
            max_wealth = cmp::max(max_wealth, client_wealth);
        }
        max_wealth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::maximum_wealth(vec![vec![1,2,3],vec![3,2,1]]);
        assert_eq!(result, 6);
    }

}
