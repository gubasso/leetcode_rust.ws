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
    let tree = Rc::new(RefCell::new(TreeNode::new(vec.get(0)?.unwrap())));
    let mut queue = VecDeque::from([Rc::clone(&tree)]);
    let mut i = 0;

    while let Some(node) = queue.pop_front() {
        i += 1;
        if let Some(&Some(val)) = vec.get(i) {
            let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().left = Some(Rc::clone(&new_node));
            queue.push_back(new_node);
        }

        i += 1;
        if let Some(&Some(val)) = vec.get(i) {
            let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().right = Some(Rc::clone(&new_node));
            queue.push_back(new_node);
        }
    }

    Some(tree)
}

struct Solution;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() { return vec![]; }
        let mut ans = Vec::new();
        let mut queue = VecDeque::from([Rc::clone(&root.unwrap())]);

        while !queue.is_empty() {
            let len = queue.len();
            let mut max = i32::MIN;

            for _ in 0..len {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                max = max.max(node.val);

                // diff ways to iterate over node leafs
                // if let Some(n) = &node.left {
                //     queue.push_back(Rc::clone(n));
                // }
                // if let Some(n) = &node.right {
                //     queue.push_back(Rc::clone(n));
                // }

                let vec = vec![&node.left, &node.right];
                for n in vec {
                    if let Some(nd) = n {
                        queue.push_back(Rc::clone(nd));
                    }
                }

            }
            ans.push(max);

        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(1),Some(3),Some(2),Some(5),Some(3),None,Some(9)],
            vec![Some(1),Some(2),Some(3)],
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
        let result = Solution::largest_values(tree);
        assert_eq!(result, vec![1,3,9]);
    }

    #[test]
    fn t2() {
        let tree = vec_to_binary_tree_iter(&inputs()[1]);
        let result = Solution::largest_values(tree);
        assert_eq!(result, vec![1,3]);
    }

}
