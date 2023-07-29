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
struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct GraphNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<GraphNode>>>,
  pub right: Option<Rc<RefCell<GraphNode>>>,
  pub parent: Option<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    GraphNode {
      val,
      left: None,
      right: None,
      parent: None,
    }
  }
}
use std::collections::{VecDeque, HashSet};
use std::rc::Rc;
use std::cell::RefCell;

type Tree = Option<Rc<RefCell<TreeNode>>>;
type Graph = Option<Rc<RefCell<GraphNode>>>;
fn vec_to_binary_tree_iter(vec: &[Option<i32>]) -> Tree {
    let tree = Rc::new(RefCell::new(TreeNode::new(vec.get(0)?.unwrap())));
    let mut i = 0;
    let mut queue = VecDeque::from([Rc::clone(&tree)]);

    while let Some(node) = queue.pop_front() {
        let TreeNode { left, right, .. } = &mut *node.borrow_mut();
        for leaf in [left, right] {
            i += 1;
            if let Some(&Some(val)) = vec.get(i) {
                let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
                *leaf = Some(Rc::clone(&new_node));
                queue.push_back(new_node);
            }
        }
    }

    Some(tree)
}

fn binary_tree_to_graph(root: Option<Rc<RefCell<TreeNode>>>) -> Graph {
    let root_rc = Rc::clone(&root?);
    let graph = Rc::new(RefCell::new(GraphNode::new(root_rc.borrow().val)));

    let mut stack: Vec<(Rc<RefCell<GraphNode>>, Rc<RefCell<TreeNode>>, Graph)> = vec![(Rc::clone(&graph), Rc::clone(&root_rc), None)];

    while let Some((graph_rc, tree_rc, self_parent)) = stack.pop() {
        let GraphNode { left, right, parent, .. } = &mut *graph_rc.borrow_mut();
        if let Some(nd) = self_parent {
            *parent = Some(Rc::clone(&nd));
        }
        if let Some(nd) = &tree_rc.borrow().left {
            let new_graph = Rc::new(RefCell::new(GraphNode::new(nd.borrow().val)));
            *left = Some(Rc::clone(&new_graph));
            stack.push((new_graph, Rc::clone(nd), Some(Rc::clone(&graph_rc))));
        }
        if let Some(nd) = &tree_rc.borrow().right {
            let new_graph = Rc::new(RefCell::new(GraphNode::new(nd.borrow().val)));
            *right = Some(Rc::clone(&new_graph));
            stack.push((new_graph, Rc::clone(nd), Some(Rc::clone(&graph_rc))));
        }
    }

    Some(graph)
}

fn get_target(node: Graph, target: i32) -> Graph {
    let node_rc = node?;
    let mut seen = HashSet::from([node_rc.as_ref().borrow().val]);
    let mut queue = VecDeque::from([Rc::clone(&node_rc)]);

    while let Some(nd) = queue.pop_front() {
        let GraphNode { val, left, right, parent } = &*nd.borrow();
        if *val == target {
            return Some(Rc::clone(&nd));
        }
        // for neighbour in [left, right, parent].into_iter().flatten() {
        //     if seen.insert(neighbour.as_ref().borrow().val) {
        //         queue.push_back(Rc::clone(neighbour));
        //     }
        // }
        for leaf in [left, right, parent] {
            if let Some(neighbour) = leaf {
                if seen.insert(neighbour.as_ref().borrow().val) {
                    queue.push_back(Rc::clone(neighbour));
                }
            }
        }
    }

    None
}

impl Solution {
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
        if root.is_none() { return vec![]; }
        let graph = binary_tree_to_graph(root);

        let target_node = get_target(graph, target.unwrap().borrow().val);
        if target_node.is_none() { return vec![]; }
        let mut seen = vec![false; 501];
        let val = target_node.as_ref().unwrap().borrow().val as usize;
        seen[val] = true;
        let mut queue = VecDeque::from([target_node.unwrap()]);
        let mut depth = 0;
        let mut ans = vec![];

        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                let nd = queue.pop_front().unwrap();
                let GraphNode { val, left, right, parent } = &*nd.borrow();
                if depth == k {
                    ans.push(*val);
                }
                // for neighbour in [left, right, parent].into_iter().flatten() {
                //     let val = neighbour.borrow().val as usize;
                //     if !seen[val] {
                //         seen[val] = true;
                //         queue.push_back(Rc::clone(neighbour));
                //     }
                // }
                for leaf in [left, right, parent] {
                    if let Some(neighbour) = leaf {
                        let val = neighbour.borrow().val as usize;
                        if !seen[val] {
                            seen[val] = true;
                            queue.push_back(Rc::clone(neighbour));
                        }
                    }
                }
            }
            if depth == k {
                return ans;
            }
            depth += 1;
        }

        ans
    }
}

fn print_graph(graph: &Graph) {
    println!("\n ## printing: ");
    let mut queue = VecDeque::from([Rc::clone(graph.as_ref().unwrap())]);
    let mut seen = HashSet::from([graph.as_ref().unwrap().borrow().val]);

    while let Some(node) = queue.pop_front() {
        let GraphNode { val, left, right, parent } = &*node.borrow();
        println!("val: {val}");
        for node in [left, right, parent].into_iter().flatten() {
            if seen.insert(node.borrow().val) {
                queue.push_back(Rc::clone(node));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Vec<Vec<Option<i32>>> {
        vec![
            vec![Some(3),Some(5),Some(1),Some(6),Some(2),Some(0),Some(8),None,None,Some(7),Some(4)],
            vec![Some(1)],
        ]
    }

    #[test]
    fn utils() {
        for vec in inputs() {
            let result = vec_to_binary_tree_iter(&vec);
            // println!("\n");
            // println!("{result:?}");
            let result = binary_tree_to_graph(result);
            let graph = result.and_then(|nd| nd.borrow_mut().left.clone());
            // print_graph(&graph);
            // println!("\n");
            // println!("{result:?}");
        }
    }

    #[test]
    fn t1() {
        let tree = vec_to_binary_tree_iter(&inputs()[0]);
        let target = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let result = Solution::distance_k(tree, target, 2);
        assert_eq!(result, vec![7,4,1]);
    }

    #[test]
    fn t2() {
        let tree = vec_to_binary_tree_iter(&inputs()[1]);
        let target = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = Solution::distance_k(tree, target, 3);
        assert_eq!(result, vec![]);
    }

}
