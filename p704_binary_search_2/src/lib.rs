#[cfg(test)]
mod tests {
    struct Solution;

    impl Solution {
        pub fn search(nums: Vec<i32>, target: i32) -> i32 {
            let mut l: usize = 0;
            let mut r: usize = nums.len() - 1;

            while l < r {
                let mid = l + (r - l + 1)/2;
                // println!("l:{l}, r:{r}, mid:{mid}");
                // println!("n[l]:{}, n[r]:{}, n[mid]:{}, t:{}", nums[l], nums[r], nums[mid], target);
                if nums[mid] == target {
                    return mid as i32;
                }
                if target < nums[mid] {
                    r = mid - 1;
                } else {
                    l = mid;
                }
            }
            if nums[l] == target {
                l as i32
            } else {
                -1
            }
        }
    }

    #[test]
    fn t1() {
        let nums = vec![-1,0,3,5,9,12];
        let target = 9;
        let output = 4;
        let result = Solution::search(nums, target);
        assert_eq!(result, output);
    }

    #[test]
    fn t2() {
        let nums = vec![-1,0,3,5,9,12];
        let target = 2;
        let output = -1;
        let result = Solution::search(nums, target);
        assert_eq!(result, output);
    }

    #[test]
    fn t3() {
        let nums = vec![5];
        let target = 5;
        let output = 0;
        let result = Solution::search(nums, target);
        assert_eq!(result, output);
    }

    #[test]
    fn t4() {
        let nums = vec![5];
        let target = -5;
        let output = -1;
        let result = Solution::search(nums, target);
        assert_eq!(result, output);
    }

}
