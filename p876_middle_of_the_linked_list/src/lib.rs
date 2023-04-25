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

fn get_list_from_vec(vec: &Vec<i32>) -> Option<Box<ListNode>> {
    let len = vec.len();
    if len == 0 { return None };
    let mut list: ListNode = ListNode::new(0);
    let mut this_val = &mut list.val;
    let mut tail = &mut list.next;

    for i in 0..len {
        *this_val = vec[i];
        *tail = if i == len-1 {
            None
        } else {
            Some(Box::new(ListNode::new(0)))
        };
        if let Some(t) = tail {
            this_val = &mut t.val;
            tail = &mut t.next;
        }
    }

    Some(Box::new(list))
}

fn get_vec_from_list(list: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
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
        if head == None { return None; }
        let mut middle = &head;
        let mut end = &head;

        while end.is_some() && end.as_ref().unwrap().next.is_some() {
            middle = &middle.as_ref().unwrap().next;
            end = &end.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        middle.clone()
    }
}

fn node(n: i32) -> Option<Box<ListNode>> {
    let mut list: ListNode = ListNode::new(1);
    let mut tail = &mut list.next;
    if n == 0 { return None; }
    if n >= 2 {
        for i in 2..=n {
            *tail = Some(Box::new(ListNode::new(i)));
            if let Some(t) = tail {
                tail = &mut t.next;
            }
        }
    }
    Some(Box::new(list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_test() {
        assert_eq!(node(0), None);
        assert_eq!(node(1), Some(Box::new(ListNode::new(1))));
        assert_eq!(node(2), Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(2))),
        })));
        assert_eq!(node(3), Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3))),
            })),
        })));
        assert_eq!(node(4), Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            })),
        })));
    }


    #[test]
    fn get_list_from_vec_test() {
        let input = vec![1];
        assert_eq!(get_list_from_vec(&input), node(1));
        let input = vec![1,2];
        assert_eq!(get_list_from_vec(&input), node(2));
        let input = vec![1,2,3];
        assert_eq!(get_list_from_vec(&input), node(3));
        let input = vec![1,2,3,4];
        assert_eq!(get_list_from_vec(&input), node(4));
    }

    #[test]
    fn get_vec_from_list_test() {
        assert_eq!(get_vec_from_list(&node(1)), vec![1]);
        assert_eq!(get_vec_from_list(&node(2)), vec![1,2]);
        assert_eq!(get_vec_from_list(&node(3)), vec![1,2,3]);
        assert_eq!(get_vec_from_list(&node(4)), vec![1,2,3,4]);
    }

    #[test]
    fn t1() {
        let input = vec![1,2,3,4,5];
        let output = vec![3,4,5];
        let list_node = get_list_from_vec(&input);
        let result_list = Solution::middle_node(list_node);
        let result_vec = get_vec_from_list(&result_list);
        assert_eq!(result_vec, output);
    }

    #[test]
    fn t2() {
        let input = vec![1,2,3,4,5,6];
        let output = vec![4,5,6];
        let list_node = get_list_from_vec(&input);
        let result_list = Solution::middle_node(list_node);
        let result_vec = get_vec_from_list(&result_list);
        assert_eq!(result_vec, output);
    }

}
