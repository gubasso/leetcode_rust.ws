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

fn vec_to_list(vec: &Vec<i32>) -> Option<Box<ListNode>> {
    if vec.is_empty() { return None }
    let mut list = Some(Box::new(ListNode::new(vec[0])));
    let mut pointer = &mut list;

    for e in vec.iter().skip(1) {
        pointer.as_mut().unwrap().next = Some(Box::new(ListNode::new(*e)));
        pointer = &mut pointer.as_mut().unwrap().next;
    }

    list
}

fn list_to_vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
    if list.is_none() { return vec![] }
    let mut vec = Vec::new();
    let mut pointer = list;

    while let Some(node) = pointer {
        vec.push(node.val);
        pointer = &node.next;
    }

    vec
}

struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut curr) = (None, head);
        while let Some(mut node) = curr {
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_utils() {
        let list = vec_to_list(&vec![1,2,3,4]);
        let vec = list_to_vec(&list);
        // println!("{list:?}");
        // println!("{vec:?}");
    }

    #[test]
    fn t1() {
        let list = vec_to_list(&vec![1,2,3,4,5]);
        let result = Solution::reverse_list(list);
        assert_eq!(list_to_vec(&result), vec![5,4,3,2,1]);
    }

    #[test]
    fn t2() {
        let list = vec_to_list(&vec![1,2]);
        let result = Solution::reverse_list(list);
        assert_eq!(list_to_vec(&result), vec![2,1]);
    }

    #[test]
    fn t3() {
        let list = vec_to_list(&vec![]);
        let result = Solution::reverse_list(list);
        assert_eq!(list_to_vec(&result), vec![]);
    }

}
