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
        let mut sum = 0;
        let mut l1 = Some(Box::new(ListNode::new(0)));
        l1.as_mut().unwrap().next = head;

        // accept the solution that clones list 2

        let mut slow = &l1.as_ref().unwrap().next;
        let mut fast = &l1.as_ref().unwrap().next;

        while fast.as_ref().is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        let mut l2 = Some(Box::new(ListNode::new(0)));
        l2.as_mut().unwrap().next = slow.clone();
        let mut curr = &mut l2.as_mut().unwrap().next;
        let mut prev: Option<Box<ListNode>> = None;

        // invert l2
        while let Some(node) = curr {

        }

        let mut l1p = &mut l1.as_mut().unwrap().next;
        let mut l2p = &mut l2.as_mut().unwrap().next;
        println!("l1: {:?}", l1p);
        println!("l2: {:?}", l2p);

        while let Some(n_l2) = l2p {
            if let Some(n_l1) = l1p {
                println!("sum: {:?}", sum);
                println!("val 1: {:?}", n_l1.val);
                println!("val 2: {:?}", n_l2.val);
                sum = sum.max(n_l1.val + n_l2.val);
                // println!("nl1: {:?}", n_l1);
                // println!("nl2: {:?}", n_l2);
            }
            l1p = &mut l1p.as_mut().unwrap().next;
            l2p = &mut l2p.as_mut().unwrap().next;
        }
        sum
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
