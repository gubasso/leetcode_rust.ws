use std::collections::HashSet;

// Definition for singly-linked list.
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
}

fn list_from_vec(vec: &Vec<i32>) -> Option<Box<ListNode>> {
    if vec.len() == 0 { return None; };
    let mut list = Some(Box::new(ListNode::new(vec[0])));
    let mut head = &mut list;

    for i in 1..vec.len() {
        if let Some(node) = head {
            node.next = Some(Box::new(ListNode::new(vec[i])));
            head = &mut node.next;
        }
    }

    list
}

fn vec_from_list(list: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut head = list;

    while let Some(node) = head {
        vec.push(node.val);
        head = &node.next;
    }

    vec
}

struct Solution;
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() { return None; }
        let mut list = head;
        let mut curr_node = list.as_mut().unwrap();

        while let Some(next_node) = curr_node.next.as_mut() {
            if next_node.val == curr_node.val {
                curr_node.next = next_node.next.take();
            } else {
                curr_node = curr_node.next.as_mut().unwrap()
            }
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let list = list_from_vec(&vec![1,1,2]);
        let result = Solution::delete_duplicates(list);
        assert_eq!(vec_from_list(&result), vec![1,2]);
    }

    #[test]
    fn t2() {
        let list = list_from_vec(&vec![1,1,2,3,3]);
        let result = Solution::delete_duplicates(list);
        assert_eq!(vec_from_list(&result), vec![1,2,3]);
    }

    #[test]
    fn t3() {
        let list = list_from_vec(&vec![]);
        let result = Solution::delete_duplicates(list);
        assert_eq!(vec_from_list(&result), vec![]);
    }

}
