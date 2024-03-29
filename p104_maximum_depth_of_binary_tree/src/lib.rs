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

fn vec_to_binary_tree(vec: &Vec<Option<i32>>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if i > vec.len() || vec[i].is_none() { return None; };
    let mut tree = TreeNode::new(vec[i].unwrap());
    tree.left = vec_to_binary_tree(&vec, 2*i + 1);
    tree.right = vec_to_binary_tree(&vec, 2*i + 2);
    Some(Rc::new(RefCell::new(tree)))
}

use std::rc::Rc;
use std::cell::RefCell;
struct Solution;
impl Solution {

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn util() {
        let binary_tree = vec_to_binary_tree(&vec![Some(3),Some(9),Some(20),None,None,Some(15),Some(7)], 0);
        // println!("{binary_tree:?}");
    }

    #[test]
    fn t1() {
        let binary_tree = vec_to_binary_tree(&vec![Some(3),Some(9),Some(20),None,None,Some(15),Some(7)], 0);
        let result = Solution::max_depth(binary_tree);
        assert_eq!(result, 3);
    }

    #[test]
    fn t2() {
        let binary_tree = vec_to_binary_tree(&vec![Some(1),None,Some(2)], 0);
        let result = Solution::max_depth(binary_tree);
        assert_eq!(result, 2);
    }

}
