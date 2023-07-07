// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

fn vec_to_binary_tree(vec: &Vec<Option<i32>>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
    let n = vec.len();
    if vec.is_empty() || i >= n || vec[i].is_none() { return None; }
    let mut tree = TreeNode::new(vec[i].unwrap());
    tree.left = vec_to_binary_tree(vec, 2*i+1);
    tree.right = vec_to_binary_tree(vec, 2*i+2);
    Some(Rc::new(RefCell::new(tree)))
}

fn vec_to_binary_tree_iter(vec: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.is_empty() || vec[0].is_none() { return None; }
    let tree = TreeNode::new(vec[0].unwrap());
    let dummy = Rc::new(RefCell::new(tree));
    let mut stack = vec![(Rc::clone(&dummy), 0)];

    while let Some((node, i)) = stack.pop() {
        let i_l = 2*i+1;
        let i_r = 2*i+2;
        if let Some(Some(e)) = vec.get(i_l) {
            let n = Rc::clone(&node);
            n.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(*e))));
            stack.push((Rc::clone(n.borrow_mut().left.as_mut().unwrap()), i_l));
        }
        if let Some(Some(e)) = vec.get(i_r) {
            let n = Rc::clone(&node);
            n.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(*e))));
            stack.push((Rc::clone(n.borrow_mut().right.as_mut().unwrap()), i_r));
        }
    }

    let inner = Rc::try_unwrap(dummy).unwrap().into_inner();
    Some(Rc::new(RefCell::new(inner)))
}

struct Solution;
impl Solution {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(1),Some(2),Some(3)],
            vec![Some(1),Some(2)],
            vec![Some(1),None,Some(2)],
            vec![Some(1),Some(2),Some(1)],
            vec![Some(1),Some(1),Some(2)],
            vec![Some(3),Some(1),Some(4),Some(3),None,Some(1),Some(5)],
            vec![Some(3),Some(3),None,Some(4),Some(2)],
        ]
    }

    #[test]
    fn util() {
        for v in inputs().iter() {
            // println!("\n");
            let tree = vec_to_binary_tree(v, 0);
            let tree_iter = vec_to_binary_tree_iter(v);
            // println!("{tree:?}");
            // println!("{tree_iter:?}");
        }
    }

    #[test]
    fn t1() {
        let p = vec_to_binary_tree(&inputs()[0], 0);
        let q = vec_to_binary_tree(&inputs()[0], 0);
        let result = Solution::is_same_tree(p, q);
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let p = vec_to_binary_tree(&inputs()[1], 0);
        let q = vec_to_binary_tree(&inputs()[2], 0);
        let result = Solution::is_same_tree(p, q);
        assert_eq!(result, false);
    }

    #[test]
    fn t3() {
        let p = vec_to_binary_tree(&inputs()[3], 0);
        let q = vec_to_binary_tree(&inputs()[4], 0);
        let result = Solution::is_same_tree(p, q);
        assert_eq!(result, false);
    }

}
