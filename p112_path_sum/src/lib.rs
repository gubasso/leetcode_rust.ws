// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
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
    if vec.get(i).map_or(true, |&e| e.is_none()) { return None; }
    let mut tree = TreeNode::new(vec[i].unwrap());
    tree.left = vec_to_binary_tree(vec, 2*i + 1);
    tree.right = vec_to_binary_tree(vec, 2*i + 2);
    Some(Rc::new(RefCell::new(tree)))
}

struct Solution;
impl Solution {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn util() {
        let vec = vec_to_binary_tree(&vec![Some(5),Some(4),Some(8),Some(11),None,Some(13),Some(4),Some(7),Some(2),None,None,None,Some(1)], 0);
        // println!("{vec:?}");
    }

    #[test]
    fn t1() {
        let btree = vec_to_binary_tree(&vec![Some(5),Some(4),Some(8),Some(11),None,Some(13),Some(4),Some(7),Some(2),None,None,None,Some(1)], 0);
        let result = Solution::has_path_sum(btree, 22);
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let btree = vec_to_binary_tree(&vec![Some(1),Some(2),Some(3)], 0);
        let result = Solution::has_path_sum(btree, 5);
        assert_eq!(result, false);
    }

    #[test]
    fn t3() {
        let btree = vec_to_binary_tree(&vec![], 0);
        let result = Solution::has_path_sum(btree, 0);
        assert_eq!(result, false);
    }

}
