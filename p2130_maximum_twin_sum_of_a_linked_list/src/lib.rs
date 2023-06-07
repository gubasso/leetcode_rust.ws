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
    if vec.is_empty() { return None; }
    let mut list = Some(Box::new(ListNode::new(vec[0])));
    let mut pointer = &mut list;
    for i in 1..vec.len() {
        pointer.as_mut().unwrap().next = Some(Box::new(ListNode::new(vec[i])));
        pointer = &mut pointer.as_mut().unwrap().next;
    };
    list
}

struct Solution;
impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut l1 = Some(Box::new(ListNode::new(0)));
        l1.as_mut().unwrap().next = head;

        // aceitar a solucao com o clone na l2

        let mut slow = &l1.as_ref().unwrap().next;
        let mut fast = &l1.as_ref().unwrap().next;

        while fast.as_ref().is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        let mut l2 = Some(Box::new(ListNode::new(0)));
        l2.as_mut().unwrap().next = slow.clone();
        println!("{l2:?}");






        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let list = vec_to_list(&vec![5,4,2,1]);
        let result = Solution::pair_sum(list);
        assert_eq!(result, 6);
    }

    #[test]
    fn t2() {
        let list = vec_to_list(&vec![1,2,3,4,6,8]);
        let result = Solution::pair_sum(list);
        assert_eq!(result, 9);
    }

}
