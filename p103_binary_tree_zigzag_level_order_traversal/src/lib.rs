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
                queue.push_back(new_node)
            }
        }
    }
    Some(tree)
}

struct Solution;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() { return vec![]; }
        let mut ans = Vec::new();
        let mut queue = VecDeque::from([root.unwrap()]);
        let mut level = 0;

        while !queue.is_empty() {
            let len = queue.len();
            let mut in_vec = Vec::new();
            level += 1;

            for _ in 0..len {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                in_vec.push(node.val);

                for leaf in [&node.left, &node.right] {
                    if let Some(nd) = leaf {
                        queue.push_back(Rc::clone(nd));
                    }
                }

                for leaf in [&node.left, &node.right].into_iter().flatten() {
                    queue.push_back(Rc::clone(leaf));
                }

            }

            if level%2 == 0 {
                in_vec.reverse();
            }

            ans.push(in_vec)
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(3),Some(9),Some(20),None,None,Some(15),Some(7)],
            vec![Some(1)],
            vec![],
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
        let result = Solution::zigzag_level_order(tree);
        let output = vec![vec![3],vec![20,9],vec![15,7]];
        assert_eq!(result, output);
    }

    #[test]
    fn t2() {
        let tree = vec_to_binary_tree_iter(&inputs()[1]);
        let result = Solution::zigzag_level_order(tree);
        let output = vec![vec![1]];
        assert_eq!(result, output);
    }

    #[test]
    fn t3() {
        let tree = vec_to_binary_tree_iter(&inputs()[2]);
        let result = Solution::zigzag_level_order(tree);
        let output: Vec<Vec<i32>> = vec![];
        assert_eq!(result, output);
    }

}
