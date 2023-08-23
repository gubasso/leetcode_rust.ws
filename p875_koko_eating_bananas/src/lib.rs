struct Solution;
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let check = |k: f64| -> bool {
            let mut hours = 0;
            for b in piles.iter() {
                let div = (*b as f64 / k).ceil() as i32;
                hours += div;
            }
            hours <= h
        };

        let (mut l, mut r) = (1, *piles.iter().max().unwrap());

        while l < r {
            let k = l + (r-l)/2;
            match check(k as f64) {
                false => l=k+1,
                true => r=k
            }
        }

        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let piles = vec![3,6,7,11];
        let h = 8;
        let ans = 4;
        let result = Solution::min_eating_speed(piles,h);
        assert_eq!(result, ans);
    }

    #[test]
    fn t2() {
        let piles = vec![30,11,23,4,20];
        let h = 5;
        let ans = 30;
        let result = Solution::min_eating_speed(piles,h);
        assert_eq!(result, ans);
    }

    #[test]
    fn t3() {
        let piles = vec![30,11,23,4,20];
        let h = 6;
        let ans = 23;
        let result = Solution::min_eating_speed(piles,h);
        assert_eq!(result, ans);
    }

    #[test]
    fn t4() {
        let piles = vec![1000000000];
        let h = 2;
        let ans = 500000000;
        let result = Solution::min_eating_speed(piles,h);
        assert_eq!(result, ans);
    }

}
