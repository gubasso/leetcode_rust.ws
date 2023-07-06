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

fn vec_to_binary_tree(vec: &Vec<Option<i32>>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.get(i).map_or(true, |&e| e.is_none()) { return None; }
    let mut tree = TreeNode::new(vec[i].unwrap());
    tree.left = vec_to_binary_tree(vec, 2*i + 1);
    tree.right = vec_to_binary_tree(vec, 2*i + 2);
    Some(Rc::new(RefCell::new(tree)))
}

// vec to binary tree iterative
fn vec_to_binary_tree_iter(vec: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.is_empty() { return None; }
    let tree = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let dummy = Rc::new(RefCell::new(tree));
    let mut stack = vec![(Rc::clone(&dummy), 0)];

    while let Some((rc_node, i)) = stack.pop() {
        let i_l = 2*i+1;
        let i_r = 2*i+2;

        if let Some(v_l) = vec.get(i_l).unwrap_or(&None) {
            println!("val left: {v_l}");
            let new_tree = Some(Rc::new(RefCell::new(TreeNode::new(*v_l))));
            let rc_cloned = Rc::clone(&rc_node);
            rc_cloned.borrow_mut().as_mut().unwrap().borrow_mut().left = new_tree;
            stack.push((Rc::new(RefCell::new(rc_cloned.borrow_mut().as_mut().unwrap().borrow_mut().left.clone())), i_l));
        }

        if let Some(v_r) = vec.get(i_r).unwrap_or(&None) {
            println!("val left: {v_r}");
            let new_tree = Some(Rc::new(RefCell::new(TreeNode::new(*v_r))));
            let rc_cloned = Rc::clone(&rc_node);
            rc_cloned.borrow_mut().as_mut().unwrap().borrow_mut().right = new_tree;
            stack.push((Rc::new(RefCell::new(rc_cloned.borrow_mut().as_mut().unwrap().borrow_mut().right.clone())), i_r));
        }

    }

    // let x = dummy.borrow().clone();
    // x
    let inner: Option<Rc<RefCell<TreeNode>>> = Rc::try_unwrap(dummy).unwrap().into_inner();
    inner
}

struct Solution;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }
        let mut ans = 0;
        let mut stack = vec![(root, i32::MIN)];

        while let Some((mut node, mut max)) = stack.pop() {
            let mut n = node.as_mut().unwrap().borrow_mut();
            if n.val >= max {
                ans += 1;
                max = n.val;
            }
            if let Some(n_l) = n.left.take() {
                stack.push((Some(n_l), max));
            }
            if let Some(n_r) = n.right.take() {
                stack.push((Some(n_r), max));
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
            vec![Some(3),Some(1),Some(4),Some(3),None,Some(1),Some(5)],
            vec![Some(3),Some(3),None,Some(4),Some(2)],
            vec![Some(1)],
        ]
    }

    #[test]
    fn util() {
        println!("\n");
        println!("recursive");
        println!("\n");
        for vec in inputs().iter() {
            println!("\n");
            println!("{vec:?}");
            let result = vec_to_binary_tree(vec, 0);
            println!("{result:?}");
        }
        println!("\n");
        println!("iterative");
        println!("\n");
        for vec in inputs().iter() {
            println!("\n");
            println!("{vec:?}");
            let result = vec_to_binary_tree_iter(vec);
            println!("{result:?}");
        }
    }

    #[test]
    fn t1() {
        let input = vec_to_binary_tree(&inputs()[0], 0);
        let result = Solution::good_nodes(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn t2() {
        let input = vec_to_binary_tree(&inputs()[1], 0);
        let result = Solution::good_nodes(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn t3() {
        let input = vec_to_binary_tree(&inputs()[2], 0);
        let result = Solution::good_nodes(input);
        assert_eq!(result, 1);
    }

}
