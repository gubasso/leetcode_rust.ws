use std::rc::Rc;
use std::cell::RefCell;

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

fn convert_vec_to_tree_node(vec: &Vec<i32>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {

    let mut node: TreeNode = TreeNode::new(vec[0]);
    let n: usize = vec.len();

    if i < n {
        node.left = convert_vec_to_tree_node(vec, 2*i + 1);
        node.right = convert_vec_to_tree_node(vec, 2*i + 2);
        node.val = vec[i];
        return Some(Rc::new(RefCell::new(node)))
    }

    None
}

pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let node_rc_refcell = root.unwrap();
    let node_val = node_rc_refcell.borrow().val;
    let left_val = node_rc_refcell.borrow().left.as_ref().unwrap().borrow().val;
    let right_val = node_rc_refcell.borrow().right.as_ref().unwrap().borrow().val;
    node_val == left_val + right_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let vec = vec![10, 4, 6];
        let result = convert_vec_to_tree_node(&vec, 0);
        assert_eq!(check_tree(result), true);
    }

    #[test]
    fn t2() {
        let vec = vec![5,3,1];
        let result = convert_vec_to_tree_node(&vec, 0);
        assert_eq!(check_tree(result), false);
    }

}
