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

fn binaty_tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut vec = Vec::new();
    let mut queue = VecDeque::new();
    let mut i = 0;

    if let Some(r) = root {
        vec.push(Some(r.borrow().val));
        queue.push_back(Rc::clone(r));
    }

    while let Some(node) = queue.pop_front() {
        let TreeNode { left, right, .. } = &*node.borrow();
        vec.extend_from_slice(&[None, None]);

        for leaf in [left, right] {
            i += 1;
            if let Some(l) = leaf {
                vec[i] = Some(l.borrow().val);
                queue.push_back(Rc::clone(l));
            }
        }
    }

    while vec.last().map_or(false, |&e| e.is_none()) {
        vec.pop();
    }

    vec
}

struct Solution;
impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(match root {
            None => Rc::new(RefCell::new(TreeNode::new(val))),
            Some(node) => {
                let rc = Rc::clone(&node);
                let TreeNode { val: v, left, right } = &mut *rc.borrow_mut();
                if val < *v {
                    *left = Self::insert_into_bst(left.take(), val);
                }
                if val > *v {
                    *right = Self::insert_into_bst(right.take(), val);
                }
                node
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(4),Some(2),Some(7),Some(1),Some(3)],
            vec![Some(40),Some(20),Some(60),Some(10),Some(30),Some(50),Some(70)],
            vec![Some(4),Some(2),Some(7),Some(1),Some(3),None,None,None,None,None,None],
        ]
    }

    fn outputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(4),Some(2),Some(7),Some(1),Some(3),Some(5)],
            vec![Some(40),Some(20),Some(60),Some(10),Some(30),Some(50),Some(70),None,None,Some(25)],
            vec![Some(4),Some(2),Some(7),Some(1),Some(3),Some(5)],
        ]
    }

    #[test]
    fn utils() {
        for vec in inputs() {
            // println!("\n");
            let result = vec_to_binary_tree_iter(&vec);
            let out = binaty_tree_to_vec(&result);
            // println!("{result:?}");
            // println!("{vec:?}");
            // println!("{out:?}");
        }
    }

    #[test]
    fn t1() {
        let in_tree = vec_to_binary_tree_iter(&inputs()[0]);
        let result = Solution::insert_into_bst(in_tree, 5);
        let result = binaty_tree_to_vec(&result);
        assert_eq!(result, outputs()[0]);
    }

    #[test]
    fn t2() {
        let in_tree = vec_to_binary_tree_iter(&inputs()[1]);
        let result = Solution::insert_into_bst(in_tree, 25);
        let result = binaty_tree_to_vec(&result);
        assert_eq!(result, outputs()[1]);
    }

    #[test]
    fn t3() {
        let in_tree = vec_to_binary_tree_iter(&inputs()[2]);
        let result = Solution::insert_into_bst(in_tree, 5);
        let result = binaty_tree_to_vec(&result);
        assert_eq!(result, outputs()[2]);
    }

}
