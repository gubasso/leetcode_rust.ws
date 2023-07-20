# p530_minimum_absolute_difference_in_bst

## Recursive approach:

Without the loop after.

```rust
// Runtime 4ms
impl Solution {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, min: &mut i32) {
        if node.is_none() { return; }
        let TreeNode { val, left, right } = &*node.as_ref().unwrap().borrow();
        Self::dfs(left, prev, min);
        if let Some(pr) = *prev {
            *min = i32::min(*min, val - pr);
        }
        *prev = Some(*val);
        Self::dfs(right, prev, min);
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min = i32::MAX;
        let mut prev = None;
        Self::dfs(&root, &mut prev, &mut min);
        min
    }
}
```

With a loop after DFS traversal to find the minimum number.

```rust
impl Solution {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if node.is_none() { return; }
        let TreeNode { val, left, right } = &*node.as_ref().unwrap().borrow();
        Self::dfs(left, vec);
        vec.push(*val);
        Self::dfs(right, vec);
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut vec = Vec::new();
        Self::dfs(&root, &mut vec);

        // let mut ans = i32::MAX;
        // Runtime 0ms
        // for i in 0..vec.len()-1 {
        //     ans = i32::min(ans, vec[i+1] - vec[i]);
        // }
        // ans

        // Runtime 4ms
        // for v in vec.windows(2) {
        //     ans = i32::min(ans, v[1] - v[0]);
        // }
        // ans

        // Runtime 4ms
        vec.windows(2).map(|v| v[1] - v[0]).min().unwrap()
    }
}
```
