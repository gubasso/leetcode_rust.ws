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
    if vec.get(0).map_or(true, |&e| e.is_none()){ return None; }
    let mut i = 0;
    let tree = Rc::new(RefCell::new(TreeNode::new(vec[i].unwrap())));
    let mut queue = VecDeque::from([Rc::clone(&tree)]);

    while i < vec.len() && !queue.is_empty() {
        let node = queue.pop_front().unwrap();
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

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<(i32, i32, i32)> {
        let TreeNode { val, left, right } = &*root.as_ref()?.borrow();
        let (max, min, diff) = match (Self::dfs(left), Self::dfs(right)) {
            (None, None) => return Some((*val, *val, 0)),
            (Some((x, n, d)), None) | (None, Some((x, n, d))) => (x, n, d),
            (Some((xl, nl, dl)), Some((xr, nr, dr))) => {
                (xl.max(xr), nl.min(nr), dl.max(dr))
            }
        };
        Some((
            max.max(*val),
            min.min(*val),
            diff.max((*val-max).abs()).max((*val-min).abs())
            ))
    }

    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root).map_or(0, |(_, _, diff)| diff)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(8),Some(3),Some(10),Some(1),Some(6),None,Some(14),None,None,Some(4),Some(7),Some(13)],
            vec![Some(1),None,Some(2),None,Some(0),Some(3)],
            vec![Some(2),Some(4),Some(3),Some(1),None,Some(0),Some(5),None,Some(6),None,None,None,Some(7)],
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
        let result = Solution::max_ancestor_diff(tree);
        assert_eq!(result, 7);
    }

    #[test]
    fn t2() {
        let tree = vec_to_binary_tree_iter(&inputs()[1]);
        let result = Solution::max_ancestor_diff(tree);
        assert_eq!(result, 3);
    }

    #[test]
    fn t3() {
        let tree = vec_to_binary_tree_iter(&inputs()[2]);
        let result = Solution::max_ancestor_diff(tree);
        assert_eq!(result, 5);
    }

}
