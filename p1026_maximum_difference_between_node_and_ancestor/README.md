# p1026_maximum_difference_between_node_and_ancestor

## Iterative


## Recursive

```rust
impl Solution {

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<(i32, i32, i32)> {
        let TreeNode { val, left, right } = &*root.as_ref()?.borrow();
        let (max, min, diff) = match (Self::dfs(left), Self::dfs(right)) {
            (None, None) => return Some((*val, *val, 0)),
            (Some((x, n, d)), None) | (None, Some((x, n, d))) => (x, n, d),
            (Some((xl, nl, dl)), Some((xr, nr, dr))) => {
                (xl.max(xr), nl.min(nr), dl.max(dr))
            }
        };
        Some((
            max.max(*val),
            min.min(*val),
            diff.max((*val-max).abs()).max((*val-min).abs())
            ))
    }

    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root).map_or(0, |(_, _, diff)| diff)
    }

}
```
