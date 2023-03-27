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
    let n: usize = vec.len();
    if n == 0 { return None; };
    let mut node: TreeNode = TreeNode::new(vec[0]);

    if i < n {
        node.left = convert_vec_to_tree_node(vec, 2*i + 1);
        node.right = convert_vec_to_tree_node(vec, 2*i + 2);
        node.val = vec[i];
        return Some(Rc::new(RefCell::new(node)))
    }

    None
}

fn convert_tree_node_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    if *root == None { return vec; };
    let root_node = root.as_ref().unwrap();
    let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![Rc::clone(&root_node)];

    while queue.len() > 0 {
        let node = queue.remove(0);
        vec.push(node.borrow().val);
        if let Some(left) = &node.borrow().left {
            queue.push(Rc::clone(left))
        };
        if let Some(right) = &node.borrow().right {
            queue.push(Rc::clone(right))
        };
    }

    vec
}

struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root == None { return None; };
        let binding = root.as_ref().unwrap().clone();
        let mut node = binding.borrow_mut();

        let left_inverted = Self::invert_tree(node.right.clone());
        let right_inverted = Self::invert_tree(node.left.clone());
        node.right = right_inverted;
        node.left = left_inverted;

        Some(Rc::clone(&binding))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        println!("\n");
        let vec_input = vec![4,2,7,1,3,6,9];
        let vec_answer = vec![4,7,2,9,6,3,1];
        let tree_input = convert_vec_to_tree_node(&vec_input, 0);
        let inv_tree_input = Solution::invert_tree(tree_input);
        let inv_vec = convert_tree_node_to_vec(&inv_tree_input);
        assert_eq!(inv_vec, vec_answer);
    }

    #[test]
    fn t2() {
        println!("\n");
        let vec_input = vec![2,1,3];
        let vec_answer = vec![2,3,1];
        let tree_input = convert_vec_to_tree_node(&vec_input, 0);
        let inv_tree_input = Solution::invert_tree(tree_input);
        let inv_vec = convert_tree_node_to_vec(&inv_tree_input);
        assert_eq!(inv_vec, vec_answer);
    }

    #[test]
    fn t3() {
        let vec_input = vec![];
        let vec_answer = vec![];
        let tree_input = convert_vec_to_tree_node(&vec_input, 0);
        let inv_tree_input = Solution::invert_tree(tree_input);
        let inv_vec = convert_tree_node_to_vec(&inv_tree_input);
        assert_eq!(inv_vec, vec_answer);
    }

}
