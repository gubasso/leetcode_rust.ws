struct Solution;
use std::cmp::Ordering::Less;
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort();
        let n = potions.len();
        let mut res = Vec::with_capacity(spells.len());

        for s in spells {
            let min_potion = success as f64/s as f64;
            // println!("## spell {}, min_potion {}", s, min_potion);
            let (mut l, mut r) = (0, n);
            while l < r {
                let m = l + (r-l)/2;
                match (potions[m] as f64).partial_cmp(&min_potion) {
                    Some(Less) => l=m+1,
                    _ => r=m,
                }
            }
            res.push((n-l) as i32);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let spells = vec![5,1,3];
        let potions = vec![1,2,3,4,5];
        let success = 7;
        let ans = vec![4,0,3];
        let result = Solution::successful_pairs(spells, potions, success);
        assert_eq!(result, ans);
    }

    #[test]
    fn t2() {
        let spells = vec![3,1,2];
        let potions = vec![8,5,8];
        let success = 16;
        let ans = vec![2,0,2];
        let result = Solution::successful_pairs(spells, potions, success);
        assert_eq!(result, ans);
    }

}
