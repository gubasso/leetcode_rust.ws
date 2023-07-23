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

fn binary_tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    if root.is_none() { return vec![]; }
    let root = root.as_ref().unwrap();
    let mut vec = vec![Some(root.borrow().val)];
    let mut queue = VecDeque::from([Rc::clone(&root)]);
    let mut i = 0;

    while let Some(node) = queue.pop_front() {
        let TreeNode { left, right, .. } = &*node.borrow();
        vec.extend_from_slice(&[None,None]);
        for leaf in [left, right] {
            i += 1;
            if let Some(l) = leaf {
                vec[i] = Some(l.borrow().val);
                queue.push_back(Rc::clone(&l));
            }
        }
    }

    while vec.last().map_or(false, |e| e.is_none()) {
        vec.pop();
    }

    vec
}

struct Solution;
impl Solution {

    pub fn closest_value(mut root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        if root.is_none() { return i32::MAX; }
        let mut ans = i32::MAX;
        let mut node = root.take();
        let ceil = target.ceil();
        let floor = target.floor();
        let ceil_diff = (ceil-target).abs();
        let floor_diff = (floor-target).abs();
        let int_target = if floor_diff <= ceil_diff {
            floor
        } else {
            ceil
        } as i32;

        while let Some(curr) = node.take() {
            let TreeNode { val, left, right } = &mut *curr.borrow_mut();
            if *val == int_target { return *val; }
            let v = *val as f64;
            let val_diff = (target - v).abs();
            let min_diff = (target - ans as f64).abs();
            if val_diff < min_diff {
                ans = *val;
            }
            node = if v > target {
                left.take()
            } else {
                right.take()
            };
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(4),Some(2),Some(5),Some(1),Some(3)],
            vec![Some(1)],
        ]
    }

    #[test]
    fn utils() {
        for v in inputs() {
            // println!("\n");
            let tree = vec_to_binary_tree_iter(&v);
            let vec = binary_tree_to_vec(&tree);
            // println!("{tree:?}");
            // println!("{v:?}");
            // println!("{vec:?}");
        }
    }

    #[test]
    fn t1() {
        let tree = vec_to_binary_tree_iter(&inputs()[0]);
        let result = Solution::closest_value(tree, 3.714286);
        assert_eq!(result, 4);
    }

    #[test]
    fn t2() {
        let tree = vec_to_binary_tree_iter(&inputs()[1]);
        let result = Solution::closest_value(tree, 4.428571);
        assert_eq!(result, 1);
    }

}
