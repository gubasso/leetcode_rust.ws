# p704_binary_search

Improved rust-like binary search:

```rust
use std::cmp::Ordering::{Equal, Less, Greater};
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0,nums.len());
        while l < r {
            let m = l + (r-l)/2;
            match nums[m].cmp(&target) {
                Equal => return m as i32,
                Less => l = m + 1,
                Greater => r = m,
            }
        }
        -1
    }
}
```


Default binary search algorithm (O(log n)) time complexity.

```rust
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r) = (0,nums.len()-1);
    while l <= r {
        let m = l + (r-l)/2;

        if nums[m] == target {
            return m as i32;
        } else if nums[m] > target {
            if m == 0 {
                return -1;
            }
            r = m - 1;
        } else if nums[m] < target {
            l = m + 1;
        }

    }
    -1
}
```
