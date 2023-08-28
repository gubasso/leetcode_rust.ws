# p78_subsets

solution as ICC teatches:

```rust
impl Solution {

    fn backtrack(nums: &Vec<i32>, ans: &mut Vec<Vec<i32>>, mut curr: Vec<i32>, i: usize) {
        if i > nums.len() {
            return;
        }
        ans.push(curr.to_vec());
        for j in i..nums.len() {
            curr.push(nums[j]);
            Self::backtrack(nums, ans, curr.to_vec(), j + 1);
            curr.pop();
        }
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // println!("\n");
        let mut ans = vec![];
        Self::backtrack(&nums, &mut ans, vec![], 0);
        // ans.sort_by_key(|v| v.len());
        ans
    }

}
```
