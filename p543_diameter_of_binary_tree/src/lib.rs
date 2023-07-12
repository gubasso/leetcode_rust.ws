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

fn vec_to_binary_tree_iter(vec: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.get(0).map_or(true, |&e| e.is_none()) { return None; }
    let mut i = 0;
    let tree = Rc::new(RefCell::new(TreeNode::new(vec[i].unwrap())));
    let mut queue = VecDeque::from([Rc::clone(&tree)]);

    while let Some(node) = queue.pop_front() {
        let mut node = node.borrow_mut();
        i += 1;
        if let Some(&Some(val)) = vec.get(i) {
            let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.left = Some(Rc::clone(&new_node));
            queue.push_back(new_node);
        }
        i+=1;
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
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>)-> (i32, i32) {
        if root.is_none() { return (0,0); }
        let (dl, cl) = Self::dfs(&root.as_ref().unwrap().borrow().left);
        let (dr, cr) = Self::dfs(&root.as_ref().unwrap().borrow().right);
        (dl.max(dr) + 1, (dl+dr).max(cl).max(cr))
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root).1
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(1),Some(2),Some(3),Some(4),Some(5)],
            vec![Some(1),Some(2)],
            vec![Some(4),Some(-7),Some(-3),None,None,Some(-9),Some(-3),Some(9),Some(-7),Some(-4),None,Some(6),None,Some(-6),Some(-6),None,None,Some(0),Some(6),Some(5),None,Some(9),None,None,Some(-1),Some(-4),None,None,None,Some(2)],
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
        let result = Solution::diameter_of_binary_tree(tree);
        assert_eq!(result, 3);
    }

    #[test]
    fn t2() {
        let tree = vec_to_binary_tree_iter(&inputs()[1]);
        let result = Solution::diameter_of_binary_tree(tree);
        assert_eq!(result, 1);
    }

    #[test]
    fn t3() {
        let tree = vec_to_binary_tree_iter(&inputs()[2]);
        let result = Solution::diameter_of_binary_tree(tree);
        assert_eq!(result, 8);
    }

}
