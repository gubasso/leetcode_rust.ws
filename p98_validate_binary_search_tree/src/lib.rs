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

use std::collections::VecDeque;
use std::ops::{Index,IndexMut};
impl Index<usize> for TreeNode {
    type Output = Option<Rc<RefCell<TreeNode>>>;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.left,
            1 => &self.right,
            n => panic!("Invalid TreeNode index: {}", n)
        }
    }
}

impl IndexMut<usize> for TreeNode {
    fn index_mut(&mut self, index: usize) -> &mut Option<Rc<RefCell<TreeNode>>> {
        match index {
            0 => &mut self.left,
            1 => &mut self.right,
            n => panic!("Invalid TreeNode index: {}", n)
        }
    }
}

fn vec_to_binary_tree_iter(vec: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let tree = Rc::new(RefCell::new(TreeNode::new(vec.get(0)?.unwrap())));
    let mut queue = VecDeque::from([Rc::clone(&tree)]);
    let mut i = 0;

    while let Some(node) = queue.pop_front() {
        for j in 0..2 {
            i += 1;
            if let Some(&Some(val)) = vec.get(i) {
                let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut()[j] = Some(Rc::clone(&new_node));
                queue.push_back(new_node);
            }
        }
    }

    Some(tree)
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;
impl Solution {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, small: i64, large: i64) -> bool {
        if node.is_none() { return true; }
        let TreeNode { val, left, right } = &*node.as_ref().unwrap().borrow();
        let val = *val as i64;
        if val <= small || val >= large {
            return false;
        }
        Self::dfs(left, small, val) && Self::dfs(right, val, large)
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root, i64::MIN, i64::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(2),Some(1),Some(3)],
            vec![Some(5),Some(1),Some(4),None,None,Some(3),Some(6)],
            vec![Some(2147483647)],
        ]
    }

    #[test]
    fn utils() {
        for vec in inputs() {
            // println!("\n");
            let result = vec_to_binary_tree_iter(&vec);
            // println!("{result:?}");
        }
    }

    #[test]
    fn t1() {
        let tree = vec_to_binary_tree_iter(&inputs()[0]);
        let result = Solution::is_valid_bst(tree);
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let tree = vec_to_binary_tree_iter(&inputs()[1]);
        let result = Solution::is_valid_bst(tree);
        assert_eq!(result, false);
    }

    #[test]
    fn t3() {
        let tree = vec_to_binary_tree_iter(&inputs()[2]);
        let result = Solution::is_valid_bst(tree);
        assert_eq!(result, true);
    }

}
