# p270_closest_binary_search_tree_value

## Recursive approach


Iterative/recursive solution: runtime 3ms

This one I thought would be more permormatic, but it had a worse runtime and I don't know why!

```rust
pub fn closest_value(mut root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
    if root.is_none() { return i32::MAX; }
    let mut ans = i32::MAX;
    let mut node = root.take();
    let ceil = target.ceil();
    let floor = target.floor();
    let ceil_diff = (ceil-target).abs();
    let floor_diff = (floor-target).abs();
    let int_target = if floor_diff <= ceil_diff {
        floor
    } else {
        ceil
    } as i32;

    while let Some(curr) = node.take() {
        let TreeNode { val, left, right } = &mut *curr.borrow_mut();
        if *val == int_target { return *val; }
        let v = *val as f64;
        let val_diff = (target - v).abs();
        let min_diff = (target - ans as f64).abs();
        if val_diff < min_diff {
            ans = *val;
        }
        node = if v > target {
            left.take()
        } else {
            right.take()
        };
    }
    ans
}
```


Better code: runtime 0ms

```rust
impl Solution {

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, target: f64, int_target: i32) -> i32 {
        if root.is_none() { return i32::MAX; }
        let TreeNode { val, left, right } = &mut *root.as_ref().unwrap().borrow_mut();
        if *val == int_target { return *val; }

        let val_f = *val as f64;
        let mut n_left = i32::MAX;
        let mut n_right = i32::MAX;

        if val_f > target {
            n_left = Self::closest_value(left.take(), target);
            if n_left == int_target { return n_left; }
        } else {
            n_right = Self::closest_value(right.take(), target);
            if n_right == int_target { return n_right; }
        }

        let val_diff = (*val as f64-target).abs();
        let left_diff = (n_left as f64 - target).abs();
        let right_diff = (n_right as f64 - target).abs();
        let min = val_diff.min(left_diff).min(right_diff);

        match min {
            _ if min == val_diff => *val,
            _ if min == left_diff => n_left,
            _ if min == right_diff => n_right,
            _ => i32::MAX,
        }
    }

    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {

        let ceil = target.ceil();
        let floor = target.floor();
        let ceil_diff = (ceil-target).abs();
        let floor_diff = (floor-target).abs();
        let int_target = if floor_diff <= ceil_diff {
            floor
        } else {
            ceil
        } as i32;

        Self::dfs(&root, target, int_target)
    }
}
```

Naive first solution: runtime 0ms

```rust
pub fn closest_value(mut root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {

    if root.is_none() { return i32::MAX; }
    let TreeNode { val, left, right } = &mut *root.as_mut().unwrap().borrow_mut();

    let ceil = target.ceil();
    let floor = target.floor();
    let ceil_diff = (ceil-target).abs();
    let floor_diff = (floor-target).abs();
    let int_target = if floor_diff <= ceil_diff {
        floor
    } else {
        ceil
    };

    if *val == int_target as i32 {
        return *val;
    }

    let n_left = Self::closest_value(left.take(), target) as f64;
    if n_left == int_target {
        return n_left as i32;
    }
    let n_right = Self::closest_value(right.take(), target) as f64;
    if n_right == int_target {
        return n_right as i32;
    }

    let val_diff = (*val as f64-target).abs();
    let left_diff = (n_left-target).abs();
    let right_diff = (n_right-target).abs();
    let min = val_diff.min(left_diff).min(right_diff);

    match min {
        _ if min == val_diff => *val,
          _ if min == left_diff => n_left as i32,
          _ if min == right_diff => n_right as i32,
          _ => i32::MAX,
    }

}
```
