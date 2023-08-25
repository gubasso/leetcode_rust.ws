struct Solution;
impl Solution {
    pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
        // println!("\n");
        // println!("{sweetness:?}, total people: {}", k+1);
        let (mut l, mut r) = (1, sweetness.iter().sum::<i32>()/(k+1));

        while l < r {
            let min_sweet = (l + r + 1)/2;
            let mut with_choc = 0;
            let mut curr_sweet = 0;
            for &s in sweetness.iter() {
                curr_sweet += s;
                if curr_sweet >= min_sweet {
                    curr_sweet = 0;
                    with_choc += 1;
                }
            }
            // println!("min_sweet: {}, with_choc: {with_choc}, l: {l}, r: {r}", min_sweet);
            if with_choc >= k + 1 {
                l = min_sweet;
            } else {
                r = min_sweet - 1;
            }
        }
        // println!("final: l: {l}, r: {r}");

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let sweetness = vec![1,2,3,4,5,6,7,8,9];
        let k = 5;
        let result = Solution::maximize_sweetness(sweetness,k);
        assert_eq!(result, 6);
    }

    #[test]
    fn t2() {
        let sweetness = vec![5,6,7,8,9,1,2,3,4];
        let k = 8;
        let result = Solution::maximize_sweetness(sweetness,k);
        assert_eq!(result, 1);
    }

    #[test]
    fn t3() {
        let sweetness = vec![1,2,2,1,2,2,1,2,2];
        let k = 2;
        let result = Solution::maximize_sweetness(sweetness,k);
        assert_eq!(result, 5);
    }

}
