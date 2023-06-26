# p104_maximum_depth_of_binary_tree

- time complexity: O(n)
    - each node is visited only once
- space complexity: O(n)
    - even in the recursive approach, the function calls are stored at memory stack

## Interactive approach

- with refcell `borrow` and `clone`
    - runtime: 2ms
    - beats: 71.68%

```
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() { return 0; }
    let mut ans = 0;
    let mut stack = Vec::from([(root.unwrap(), 1)]);

    while let Some((node_rc, depth)) = stack.pop() {
        ans = ans.max(depth);
        let node_rc_borrow = node_rc.borrow();
        if let Some(left) = &node_rc_borrow.left {
            stack.push((left.clone(), depth + 1));
        }
        if let Some(right) = &node_rc_borrow.right {
            stack.push((right.clone(), depth + 1));
        }
    }

    ans
}
```

- with refcell `borrow_mut` and `take`
    - runtime: 0ms
    - beats 100%

```
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() { return 0; }
    let mut ans = 0;
    let mut stack = Vec::from([(root.unwrap(), 1)]);

    while let Some((node_rc, depth)) = stack.pop() {
        ans = ans.max(depth);
        let mut node_rc_mut = node_rc.borrow_mut();
        if let Some(left) = node_rc_mut.left.take() {
            stack.push((left, depth + 1));
        }
        if let Some(right) = node_rc_mut.right.take() {
            stack.push((right, depth + 1));
        }
    }

    ans
}
```

## Recursive approach

```
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() { return 0; }
    let root_node = root.unwrap();
    let left = Solution::max_depth(root_node.borrow().left.clone());
    let right = Solution::max_depth(root_node.borrow().right.clone());
    1 + left.max(right)
}
```
