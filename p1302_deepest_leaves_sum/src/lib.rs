use std::ops::{Index,IndexMut};
use std::collections::VecDeque;
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
    let mut i = 0;
    let mut queue = VecDeque::from([Rc::clone(&tree)]);

    while let Some(node) = queue.pop_front() {
        let mut node = node.borrow_mut();

        for j in 0..=1 {
            i += 1;
            if let Some(&Some(val)) = vec.get(i) {
                let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
                node[j] = Some(Rc::clone(&new_node));
                queue.push_back(new_node);
            }
        }

    }

    Some(tree)
}

struct Solution;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }
        let mut queue = VecDeque::from([root.unwrap()]);
        let mut sum = 0;

        while !queue.is_empty() {
            let len = queue.len();
            sum = 0;

            for _ in 0..len {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                sum += node.val;
                for j in 0..=1 {
                    if let Some(nd) = &node[j] {
                        queue.push_back(Rc::clone(&nd));
                    }
                }
            }

        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(1),Some(2),Some(3),Some(4),Some(5),None,Some(6),Some(7),None,None,None,None,Some(8)],
            vec![Some(6),Some(7),Some(8),Some(2),Some(7),Some(1),Some(3),Some(9),None,Some(1),Some(4),None,None,None,Some(5)],
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
        let result = Solution::deepest_leaves_sum(tree);
        assert_eq!(result, 15);
    }

    #[test]
    fn t2() {
        let tree = vec_to_binary_tree_iter(&inputs()[1]);
        let result = Solution::deepest_leaves_sum(tree);
        assert_eq!(result, 19);
    }

}
