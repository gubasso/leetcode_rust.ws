use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans: i32 = i32::MAX;
        for (i, n) in cards.iter().enumerate() {
            let val = map.entry(*n).or_insert(i as i32);
            let diff = i as i32 - *val;
            if diff > 0 {
                ans = i32::min(ans, diff + 1);
            }
            *val = i as i32;
        }
        if ans == i32::MAX { -1 } else { ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::minimum_card_pickup(vec![3,4,2,3,4,7]);
        assert_eq!(result, 4);
    }

    #[test]
    fn t2() {
        let result = Solution::minimum_card_pickup(vec![1,0,5,3]);
        assert_eq!(result, -1);
    }

    #[test]
    fn t3() {
        let result = Solution::minimum_card_pickup(vec![95,11,8,65,5,86,30,27,30,73,15,91,30,7,37,26,55,76,60,43,36,85,47,96,6]);
        assert_eq!(result, 3);
    }

    #[test]
    fn t4() {
        let result = Solution::minimum_card_pickup(vec![77,10,11,51,69,83,33,94,0,42,86,41,65,40,72,8,53,31,43,22,9,94,45,80,40,0,84,34,76,28,7,79,80,93,20,82,36,74,82,89,74,77,27,54,44,93,98,44,39,74,36,9,22,57,70,98,19,68,33,68,49,86,20,50,43]);
        assert_eq!(result, 3);
    }

}
