# p701_insert_into_a_binary_search_tree

## Recursive approach:

Solution 1: runtime 5ms

```rust
pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
    let root = root.unwrap();
    let tree = Rc::clone(&root);
    let TreeNode { val: rval, left, right } = &mut *tree.borrow_mut();

    if val < *rval {
        *left = Self::insert_into_bst(left.take(), val);
    }

    if val > *rval {
        *right = Self::insert_into_bst(right.take(), val);
    }

    Some(root)
}
```

Solution 2: runtime 11ms

```rust
pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(match root {
            None => Rc::new(RefCell::new(TreeNode::new(val))),
            Some(node) => {
            let rc = Rc::clone(&node);
            let TreeNode { val: v, left, right } = &mut *rc.borrow_mut();
            if val < *v {
            *left = Self::insert_into_bst(left.take(), val);
            }
            if val > *v {
            *right = Self::insert_into_bst(right.take(), val);
            }
            node
            }
            })
}
```
