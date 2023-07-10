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
    if vec.get(i).map_or(true, |&e| e.is_none()) { return None; }
    let mut tree = TreeNode::new(vec[i].unwrap());
    tree.left = vec_to_binary_tree(vec, 2*i+1);
    tree.right = vec_to_binary_tree(vec, 2*i+2);
    Some(Rc::new(RefCell::new(tree)))
}

fn vec_to_binary_tree_iter(vec: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.get(0).map_or(true, |&e| e.is_none()) { return None; }
    let tree = Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap())));
    let mut stack = vec![(Rc::clone(&tree), 0)];

    while let Some((node, i)) = stack.pop() {
        let mut n = node.borrow_mut();
        let i_l = 2*i+1;
        let i_r = 2*i+2;
        if let Some(&Some(val)) = vec.get(i_l) {
            let new_n = Rc::new(RefCell::new(TreeNode::new(val)));
            n.left = Some(Rc::clone(&new_n));
            stack.push((Rc::clone(&new_n), i_l));
        }
        if let Some(&Some(val)) = vec.get(i_r) {
            let new_n = Rc::new(RefCell::new(TreeNode::new(val)));
            n.right = Some(Rc::clone(&new_n));
            stack.push((Rc::clone(&new_n), i_r));
        }
    }

    Some(tree)
}

struct Solution;
impl Solution {
    pub fn lowest_common_ancestor(mut root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() { return None; }
        let root_val = root.as_ref().unwrap().borrow().val;
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        if root_val == p_val || root_val == q_val { return root; }

        let left = Solution::lowest_common_ancestor(root.as_mut().unwrap().borrow_mut().left.take(), p.clone(), q.clone());
        let right = Solution::lowest_common_ancestor(root.as_mut().unwrap().borrow_mut().right.take(), p.clone(), q.clone());

        match (left, right) {
            (Some(_), Some(_)) => root,
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            _ => None,
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(3),Some(5),Some(1),Some(6),Some(2),Some(0),Some(8),None,None,Some(7),Some(4)],
            vec![Some(1),Some(2)],
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
        for vec in inputs().iter() {
            // println!("\n");
            let list = vec_to_binary_tree(vec, 0);
            let list_iter = vec_to_binary_tree_iter(vec);
            // println!("{list:?}");
            // println!("{list_iter:?}");
        }
    }

    #[test]
    fn t1() {
        let tree = vec_to_binary_tree(&inputs()[0], 0);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = Solution::lowest_common_ancestor(tree, p, q);
        assert_eq!(result.unwrap().borrow().val, 3);
    }

    #[test]
    fn t2() {
        let tree = vec_to_binary_tree(&inputs()[0], 0);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let result = Solution::lowest_common_ancestor(tree, p, q);
        assert_eq!(result.unwrap().borrow().val, 5);
    }

    #[test]
    fn t3() {
        let tree = vec_to_binary_tree(&inputs()[1], 0);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let result = Solution::lowest_common_ancestor(tree, p, q);
        assert_eq!(result.unwrap().borrow().val, 1);
    }

}
