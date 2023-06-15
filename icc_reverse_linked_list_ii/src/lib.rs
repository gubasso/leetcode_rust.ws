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
    let mut curr = list;

    while let Some(node) = curr {
        vec.push(node.val);
        curr = &node.next;
    }

    vec
}

struct Solution;
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {

        let mut dummy = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));

        let mut before = &mut dummy;
        for _ in 1..left {
            before = &mut before.as_mut()?.next;
        }

        let mut prev = before.as_mut()?.next.take();
        let mut curr = prev.as_mut()?.next.take();

        for _ in left..right {
            let next = curr.as_mut()?.next.take();
            curr.as_mut()?.next = prev.take();
            prev = curr;
            curr = next;
        }

        let mut rev_tail = &mut prev;
        for _ in left..right {
            rev_tail = &mut rev_tail.as_mut()?.next;
        }

        rev_tail.as_mut()?.next = curr;
        before.as_mut()?.next = prev;

        dummy?.next
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
        let result = Solution::reverse_between(list, 2, 4);
        assert_eq!(list_to_vec(&result), vec![1,4,3,2,5]);
    }

    #[test]
    fn t2() {
        let list = vec_to_list(&vec![5]);
        let result = Solution::reverse_between(list, 1, 1);
        assert_eq!(list_to_vec(&result), vec![5]);
    }

}
