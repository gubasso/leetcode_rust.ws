struct Solution;
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        // println!("\n");
        let (mut l, mut r) = (0,0);
        for &n in nums.iter() {
            r += n;
            l = l.max(n);
        }

        while l < r {
            let max_sum = l + (r-l)/2;
            // println!("# max sum: {max_sum}, l: {l}, r: {r}");
            let mut chunks = 0;
            let mut chunk_sum = 0;
            for &n in nums.iter() {
                chunk_sum += n;
                if chunk_sum > max_sum {
                    // println!("chunk_sum: {chunk_sum}");
                    chunks += 1;
                    chunk_sum = n;
                    // println!("chunks: {chunks} new chunk_sum: {chunk_sum}");
                }
            }
            chunks += 1;

            // println!("final chunks: {chunks}...");
            if chunks > k {
                l = max_sum + 1;
            } else {
                r = max_sum;
            }
            // println!("final l: {l}, r: {r}");
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![7,2,5,10,8];
        let k = 2;
        let result = Solution::split_array(nums,k);
        assert_eq!(result, 18);
    }

    #[test]
    fn t2() {
        let nums = vec![1,2,3,4,5];
        let k = 2;
        let result = Solution::split_array(nums,k);
        assert_eq!(result, 9);
    }

    #[test]
    fn t3() {
        let nums = vec![1,4,4];
        let k = 3;
        let result = Solution::split_array(nums,k);
        assert_eq!(result, 4);
    }

}
