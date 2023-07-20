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

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

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

struct Solution;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root.map(|node| Rc::try_unwrap(node).unwrap().into_inner()) {
            None => 0,
            Some(TreeNode { val, left, right }) => {
                if val < low {
                    Self::range_sum_bst(right, low, high)
                } else if val > high {
                    Self::range_sum_bst(left, low, high)
                } else {
                    val + Self::range_sum_bst(left, low, high)
                        + Self::range_sum_bst(right, low, high)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(10),Some(5),Some(15),Some(3),Some(7),None,Some(18)],
            vec![Some(10),Some(5),Some(15),Some(3),Some(7),Some(13),Some(18),Some(1),None,Some(6)],
        ]
    }

    #[test]
    fn utils() {
        for vec in inputs().iter() {
            // println!("\n");
            let result = vec_to_binary_tree_iter(vec);
            // println!("{result:?}");
        }
    }

    #[test]
    fn t1() {
        let tree = vec_to_binary_tree_iter(&inputs()[0]);
        let result = Solution::range_sum_bst(tree, 7, 15);
        assert_eq!(result, 32);
    }

    #[test]
    fn t2() {
        let tree = vec_to_binary_tree_iter(&inputs()[1]);
        let result = Solution::range_sum_bst(tree, 6, 10);
        assert_eq!(result, 23);
    }

}
