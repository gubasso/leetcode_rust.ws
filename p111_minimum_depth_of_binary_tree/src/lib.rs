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

type OptNode = Option<Rc<RefCell<TreeNode>>>;

fn vec_to_binary_tree(vec: &Vec<Option<i32>>, i: usize) -> OptNode {
    let el = vec.get(i);
    if el.map_or(true, |&e| e.is_none()) { return None; }
    let mut tree = TreeNode::new(el.unwrap().unwrap());
    tree.left = vec_to_binary_tree(vec, 2*i+1);
    tree.right = vec_to_binary_tree(vec, 2*i+2);
    Some(Rc::new(RefCell::new(tree)))
}

fn vec_to_binary_tree_iter(vec: &Vec<Option<i32>>) -> OptNode {
    let first = vec.first();
    if first.map_or(true, |&e| e.is_none()) { return None; }
    let tree = Rc::new(RefCell::new(TreeNode::new(first.unwrap().unwrap())));
    let mut stack = vec![(Rc::clone(&tree), 0)];

    while let Some((node_rc, i)) = stack.pop() {
        let mut n = node_rc.borrow_mut();
        let i_l = 2*i+1;
        let i_r = 2*i+2;
        if let Some(&Some(e)) = vec.get(i_l) {
            let new_node = Rc::new(RefCell::new(TreeNode::new(e)));
            n.left = Some(Rc::clone(&new_node));
            stack.push((new_node, i_l));
        }
        if let Some(&Some(e)) = vec.get(i_r) {
            let new_node = Rc::new(RefCell::new(TreeNode::new(e)));
            n.right = Some(Rc::clone(&new_node));
            stack.push((new_node, i_r));
        }
    }

    Some(tree)
}

struct Solution;
impl Solution {

    fn dfs(root: &OptNode) -> i32 {
        if root.is_none() { return 0; }
        let left = Self::dfs(&root.as_ref().unwrap().borrow().left);
        let right = Self::dfs(&root.as_ref().unwrap().borrow().right);
        if left == 0 { return 1+right; }
        if right == 0 { return 1+left; }
        1 + left.min(right)
    }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(3),Some(9),Some(20),None,None,Some(15),Some(7)],
            // this example is wrong. This array does not represent a single legged treenode
            // vec![Some(2),None,Some(3),None,Some(4),None,Some(5),None,Some(6)],
        ]
    }

    #[test]
    fn utils() {
        for vec in inputs().iter() {
            let result = vec_to_binary_tree(vec, 0);
            let result_iter = vec_to_binary_tree_iter(vec);
            assert_eq!(result, result_iter);
        }
    }

    #[test]
    fn t1() {
        let tree = vec_to_binary_tree(&inputs()[0], 0);
        let result = Solution::min_depth(tree);
        assert_eq!(result, 2);
    }

    // #[test]
    // fn t2() {
    //     let tree = vec_to_binary_tree_iter(&inputs()[1]);
    //     // println!("{tree:?}");
    //     let result = Solution::min_depth(tree);
    //     assert_eq!(result, 5);
    // }

}
