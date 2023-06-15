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
    for e in vec.iter().skip(1) {
        pointer.as_mut().unwrap().next = Some(Box::new(ListNode::new(*e)));
        pointer = &mut pointer.as_mut().unwrap().next;
    };
    list
}

struct Solution;
impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut sum = 0;

        // accept the solution that clones list 2
        let mut slow = &head;
        let mut fast = &head;

        while fast.as_ref().is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        let mut curr = slow.clone();
        let mut prev: Option<Box<ListNode>> = None;

        // reverse / invert l2
        while let Some(mut node) = curr {
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }

        let mut p1 = &head;
        let mut p2 = &prev;
        while let Some(n2) = p2 {
            if let Some(n1) = p1 {
                sum = sum.max(n1.val + n2.val);
                p1 = &n1.next;
                p2 = &n2.next;
            }
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

    #[test]
    fn t3() {
        let list = vec_to_list(&vec![4,2,2,3]);
        let result = Solution::pair_sum(list);
        assert_eq!(result, 7);
    }

    #[test]
    fn t4() {
        let list = vec_to_list(&vec![1,100000]);
        let result = Solution::pair_sum(list);
        assert_eq!(result, 100001);
    }

}
