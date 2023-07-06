# p1448_count_good_nodes_in_binary_tree

## Iterative solution (more performatic)

- https://leetcode.com/problems/count-good-nodes-in-binary-tree/submissions/987987374/

```rust
pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut stack = vec![(root, i32::MIN)];
    let mut rez = 0;

    while let Some(tuple) = stack.pop() {
        // one destructuring multiple structures
        // 1. tuple to its values
        // 2. Option to Some() and internal content
        if let (Some(node_rc), mut max_val) = tuple {
            let mut node_ref = node_rc.borrow_mut();
            if node_ref.val >= max_val {
                rez += 1;
                max_val = node_ref.val;
            }
            stack.push((node_ref.left.take(), max_val));
            stack.push((node_ref.right.take(), max_val));
        }
    }
    
    rez
}
```

## Iterative solution

```rust
pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() { return 0; }
    let mut ans = 0;
    let mut stack = vec![(root, i32::MIN)];

    while let Some((mut node, mut max)) = stack.pop() {
        let mut n = node.as_mut().unwrap().borrow_mut();
        if n.val >= max {
            ans += 1;
            max = n.val;
        }
        if let Some(n_l) = n.left.take() {
            stack.push((Some(n_l), max));
        }
        if let Some(n_r) = n.right.take() {
            stack.push((Some(n_r), max));
        }
    }

    ans
}
```

## Recursive solution

```rust
pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut max_so_far: f32) -> f32 {
        let mut ans = 0.0;
        if node.is_none() { return 0.0 }
        let n = node.as_ref().unwrap().borrow_mut();
        let val = n.val as f32;
        if val >= max_so_far {
            ans += 1.0;
            max_so_far = val;
        }
        let left = dfs(&n.left, max_so_far);
        let right = dfs(&n.right, max_so_far);
        ans + left + right
    }
    dfs(&root, f32::NEG_INFINITY) as i32
}
```
