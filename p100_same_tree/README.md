# p100_same_tree

<!-- toc GFM -->

* [Iterative approach](#iterative-approach)
* [Recursive approaches / solutions](#recursive-approaches--solutions)
    - [My first recursive solution](#my-first-recursive-solution)
    - [Inspired by leetcode solution](#inspired-by-leetcode-solution)

<!-- toc -->

- time complexity: O(n)
    - in the worst case scenario, each node will be visited once
- space complexity: O(n)
    - in the worst case scenario, and in a unbalanced tree, the stack will grow to the size of N

## Iterative approach

## Recursive approaches / solutions

- runtime: 0ms

```rust
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack = vec![(p, q)];

    while let Some((m, n)) = stack.pop() {
        match (m, n) {
            (None, None) => continue,
            (Some(x), Some(y)) => {
                let mut x = x.borrow_mut();
                let mut y = y.borrow_mut();
                if x.val != y.val { return false; }
                stack.push((x.left.take(), y.left.take()));
                stack.push((x.right.take(), y.right.take()));
            },
            _ => return false,
        }
    }

    true
}
```

### My first recursive solution

```rust
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() && q.is_none() { return true; }
    if p.is_none() || q.is_none() { return false; }
    if p.as_ref().unwrap().borrow().val != q.as_ref().unwrap().borrow().val { return false; }
    let left = Solution::is_same_tree(p.as_ref().unwrap().borrow().left.clone(), q.as_ref().unwrap().borrow().left.clone());
    let right = Solution::is_same_tree(p.as_ref().unwrap().borrow().right.clone(), q.as_ref().unwrap().borrow().right.clone());
    left && right
}
```

### Inspired by leetcode solution

- https://leetcode.com/problems/same-tree/solutions/3027498/checkout-this-rust-solution-for-reference/

```rust
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(ref m), Some(ref n)) => {
            let m = m.borrow();
            let n = n.borrow();
            m.val == n.val &&
                Self::is_same_tree(m.left.clone(), n.left.clone()) &&
                Self::is_same_tree(m.right.clone(), n.right.clone())
        },
        _ => false,
    }
}
```
