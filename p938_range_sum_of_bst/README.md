# p938_range_sum_of_bst

- time complexity: O(n)
- space complexity: O(n)

## Recursive approach

Solution 1:

- runtime 17ms

```rust
pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    match root.map(|node| Rc::try_unwrap(node).unwrap().into_inner()) {
        None => 0,
        Some(TreeNode { val, left, right }) => {
            if val < low {
                Self::range_sum_bst(right, low, high)
            } else if val > high {
                Self::range_sum_bst(left, low, high)
            } else {
                val + Self::range_sum_bst(left, low, high)
                    + Self::range_sum_bst(right, low, high)
            }
        }
    }
}
```

Solution 2:

- Runtime: 16 ms

```rust
pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    match root {
        Some(rt) => {
            let mut node = rt.borrow_mut();
            let val = node.val;
            let left = node.left.take();
            let right = node.right.take();
            if low > val {
                return Self::range_sum_bst(right, low, high);
            }
            if val > high {
                return Self::range_sum_bst(left, low, high);
            }
            val + Self::range_sum_bst(left, low, high) + Self::range_sum_bst(right, low, high)
        },
            None => 0,
    }
}
```

