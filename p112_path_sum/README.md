# p112_path_sum

- time complexity: O(n)
    - each node is visited exactly once
    - where N is the number of nodes
- space complexity: O(n)
    - in the worst case, the tree is unbalanced
    - each node has only one child node
    - the recursion call would occur N times (the height of the tree)
    - therefore the storage to keep the call stack would be O(n)
    - but in the best case, the tree is completily balanced
    - the height of the tree would be log(n)
    - therefore, the space complexity in the best case scenario would be O(log(n))

## Iterative approach

- runtime: 0ms

```rust
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root.is_none() { return false; }
    let mut stack = vec![(root, target_sum)];

    while let Some((mut node, sum)) = stack.pop() {
        let mut n = node.as_mut().unwrap().borrow_mut();
        let curr = sum - n.val;
        if n.left.is_none() && n.right.is_none() && curr == 0 {
            return true;
        }
        if let Some(left_node) = n.left.take() {
            stack.push((Some(left_node), curr));
        }
        if let Some(right_node) = n.right.take() {
            stack.push((Some(right_node), curr));
        }
    }

    false
}
```

## Recursive approach (after consult leetcode solutions)

- runtime: 3ms

with take

```rust
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if let Some(node) = root {
        let mut n = node.borrow_mut();
        if n.left.is_none() && n.right.is_none() { return n.val == target_sum; }
        return Solution::has_path_sum(n.left.take(), target_sum - n.val) ||
            Solution::has_path_sum(n.right.take(), target_sum - n.val)
    }
    false
}
```

with clone

```rust
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if let Some(node) = root {
        let n = node.borrow();
        if n.left.is_none() && n.right.is_none() { return n.val == target_sum; }
        return Solution::has_path_sum(n.left.clone(), target_sum - n.val) ||
            Solution::has_path_sum(n.right.clone(), target_sum - n.val)
    }
    false
}
```

## My naive first implementation: recursive

- runtime: 3ms

```rust
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    fn dfs (node_opt: &Option<Rc<RefCell<TreeNode>>>, mut curr: i32, target_sum: i32) -> bool {
        if node_opt.is_none() { return false; }
        let node = &node_opt.as_ref().unwrap();
        curr += node.borrow().val;

        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            if curr == target_sum { return true; };
        }

        let left = dfs(&node.borrow().left, curr, target_sum);
        let right = dfs(&node.borrow().right, curr, target_sum);

        left || right
    }

    dfs(&root, 0, target_sum)
}
```
