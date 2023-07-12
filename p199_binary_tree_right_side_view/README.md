# p199_binary_tree_right_side_view

## BFS, iterative

- https://leetcode.com/problems/binary-tree-right-side-view/solutions/3541829/rust-o-n-solution-using-bfs-breadth-first-search/

Nice way to traverse a tree but only with references, not changing the internal content

**tree node to vec**

```rust
fn convert_tree_node_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    if *root == None { return vec; };
    let root_node = root.as_ref().unwrap();
    let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![Rc::clone(&root_node)];

    while queue.len() > 0 {
        let node = queue.remove(0);
        vec.push(node.borrow().val);
        if let Some(left) = &node.borrow().left {
            queue.push(Rc::clone(left))
        };
        if let Some(right) = &node.borrow().right {
            queue.push(Rc::clone(right))
        };
    }

    vec
}
```

**p199_binary_tree_right_side_view**

```rust
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() { return vec![]; }
    let mut ans = vec![];
    let mut queue = VecDeque::from([root.unwrap()]);

    while !queue.is_empty() {
        let len = queue.len();
        for i in 0..len {
            let node = queue.pop_front().unwrap();

            // let left = &node.borrow().left;
            // if left.is_some() {
            //     queue.push_back(Rc::clone(left.as_ref().unwrap()));
            // }
            if let Some(node) = &node.borrow().left {
                queue.push_back(Rc::clone(node));
            }

            // let right = &node.borrow().right;
            // if right.is_some() {
            //     queue.push_back(Rc::clone(right.as_ref().unwrap()));
            // }
            if let Some(node) = &node.borrow().right {
                queue.push_back(Rc::clone(node));
            }

            if i == len - 1 {
                ans.push(node.borrow().val);
            }
        }
    }

    ans
}
```
