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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut fast, mut slow) = (&head, &head);
            while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
                slow = &slow.as_ref().unwrap().next;
                fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            }
        slow.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let list = list_from_vec(&vec![1,2,3,4,5]);
        let result = Solution::middle_node(list);
        assert_eq!(vec_from_list(&result), vec![3,4,5]);
    }

    #[test]
    fn t2() {
        let list = list_from_vec(&vec![1,2,3,4,5,6]);
        let result = Solution::middle_node(list);
        assert_eq!(vec_from_list(&result), vec![4,5,6]);
    }

}
