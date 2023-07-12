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
use std::rc::Rc;
use std::cell::RefCell;

fn vec_to_binary_tree_iter(vec: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    vec.get(0)?;
    let mut i = 0;
    let tree = Rc::new(RefCell::new(TreeNode::new(vec[i]?)));
    let mut queue = VecDeque::from([Rc::clone(&tree)]);

    while let Some(node) = queue.pop_front() {
        let mut node = node.borrow_mut();
        i += 1;
        if let Some(&Some(val)) = vec.get(i) {
            let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.left = Some(Rc::clone(&new_node));
            queue.push_back(new_node);
        }
        i += 1;
        if let Some(&Some(val)) = vec.get(i) {
            let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.right = Some(Rc::clone(&new_node));
            queue.push_back(new_node);
        }
    }

    Some(tree)
}

struct Solution;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() { return vec![]; }
        let mut ans = vec![];
        let mut queue = VecDeque::from([root.unwrap()]);

        while !queue.is_empty() {
            let len = queue.len();
            for i in 0..len {
                let node = queue.pop_front().unwrap();

                if let Some(node) = &node.borrow().left {
                    queue.push_back(Rc::clone(node));
                }

                // if let Some(node) = &node.borrow().right {
                //     queue.push_back(Rc::clone(node));
                // }
                if let Some(ref node) = node.borrow().right {
                    queue.push_back(Rc::clone(node));
                }

                if i == len - 1 {
                    ans.push(node.borrow().val);
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(1),Some(2),Some(3),None,Some(5),None,Some(4)],
            vec![Some(1),None,Some(3)],
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
        let result = Solution::right_side_view(tree);
        assert_eq!(result, vec![1,3,4]);
    }

    #[test]
    fn t2() {
        let tree = vec_to_binary_tree_iter(&inputs()[1]);
        let result = Solution::right_side_view(tree);
        assert_eq!(result, vec![1,3]);
    }

    #[test]
    fn t3() {
        let tree = vec_to_binary_tree_iter(&inputs()[2]);
        let result = Solution::right_side_view(tree);
        assert_eq!(result, vec![]);
    }

}
