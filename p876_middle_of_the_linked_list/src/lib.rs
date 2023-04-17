// Definition for singly-linked list.

use std::mem;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {

  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }

  fn push(&mut self, val: i32) {
      let new_node = ListNode {
          val,
          next: Some(Box::new(mem::replace(self, ListNode::new(0)))),
      };
      *self = new_node;
  }

  fn pop(&mut self) -> Option<Box<ListNode>> {
    match mem::replace(&mut self.next, None) {
        None => None,
        Some(next_node) => {
            *self = *next_node;
            Some(Box::new(ListNode::new(self.val)))
        }
    }
  }

}

// struct Solution;
//
// impl Solution {
//     pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn node_1() -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            val: 1,
            next: None,
        }))
    }

    fn node_2() -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            val: 2,
            next: node_1(),
        }))
    }

    fn node_3() -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            val: 3,
            next: node_2(),
        }))
    }

    fn node_4() -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            val: 4,
            next: node_3(),
        }))
    }

    #[test]
    fn t1() {
        let input = vec![1,2,3,4,5];
        let output = vec![3,4,5];
        let list_node = get_list_from_vec(input);
        // println!("list_node: {:?}", list_node);
        let result_list = Solution::middle_node(list_node);
        // println!("result_list: {:?}", result_list);
        let result_vec = get_vec_from_list(result_list);
        // println!("result_vec: {:?}", result_vec);
        assert_eq!(result_vec, output);
    }

    // #[test]
    // fn t2() {
    //    let input = vec![1,2,3,4,5,6];
    //     let output = vec![4,5,6];
    //     let list_node = get_list_from_vec(input);
    //     let result_list = Solution::middle_node(list_node);
    //     let result_vec = get_vec_from_list(result_list);
    //     assert_eq!(result_vec, output);
    // }

}
