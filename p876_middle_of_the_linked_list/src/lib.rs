//l Definition for singly-linked list.

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

fn get_list_from_vec(vecin: Vec<i32>) -> Option<Box<ListNode>> {
    let mut vec = vecin.clone();
    if vec.len() == 0 { return None; }
    println!("vecin: {:?}", &vecin);
    println!("vec.len: {:?}", &vecin.len());
    if vec.len() == 1 {
        Some(Box::new(ListNode { val: vec[0], next: None }))
    } else {
        println!("else case, will pop vec: {:?}", &vec);
        let val = vec.pop().unwrap();
        Some(Box::new(ListNode { val, next: get_list_from_vec(vec)}))
    }

}

fn get_vec_from_list(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let mut node = list.clone();

    loop {
        let node_ok = node.as_ref().unwrap();
        vec.push(node_ok.val);
        if node_ok.next == None {
            break;
        } else {
            node = node_ok.next.clone();
        }
    }

    vec
}

fn len_list(list: &Option<Box<ListNode>>) -> i32 {
    let mut len: i32 = 1;
    if list.as_ref().unwrap().next != None {
        len += len_list(&list.as_ref().unwrap().next)
    }
    len
}

fn node_until_n(n: i32, list: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut node_ok = list.clone().unwrap();
    let m = node_ok.val;

    if n == m {
        Some(Box::new(ListNode {
            val: m,
            next: None,
        }))
    } else {
        node_until_n(
            n,
            &Some(Box::new(ListNode {
                        val: m,
                        next: node_from_n(
                            n,
                            &get_list_from_vec((m-1..=1).collect())
                        ),
                    }
                )
            )
        )
    }
}

fn node_from_n(n: i32, list: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut node_in = list.clone();
    if let Some(node) = node_in {
        while node.next != None {
            if n == node.val {
                break;
            } else {
                node_in = node.next;
            }
        }
    }
    node_in
}


struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = len_list(&head);
        let middle_val = len/2+1;
        let mut new_node = head.clone();
        // let mut next_node = new_node.as_ref().unwrap().next;
        let middle_node = node_until_n(middle_val, &new_node);
        println!("middle_node {:?}: ", middle_node);

        new_node
    }
}

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

    #[test]
    fn test_get_list_from_vec() {
        let input = vec![1];
        let output = node_1();
        let result_list = get_list_from_vec(input);
        assert_eq!(result_list, output);
        let input = vec![1,2];
        let output = node_2();
        let result_list = get_list_from_vec(input);
        assert_eq!(result_list, output);
    }

    #[test]
    fn test_get_vec_from_list() {
       let input = node_1();
       let output = vec![1];
       let result_vec = get_vec_from_list(input);
       assert_eq!(result_vec, output);
       let input = node_2();
       let output = vec![2,1];
       let result_vec = get_vec_from_list(input);
       assert_eq!(result_vec, output);
       let input = node_3();
       let output = vec![3,2,1];
       let result_vec = get_vec_from_list(input);
       assert_eq!(result_vec, output);
    }

    #[test]
    fn test_len_list() {
       let input = node_1();
       let output = 1;
       let len = len_list(&input);
       assert_eq!(len, output);
       let input = node_2();
       let output = 2;
       let len = len_list(&input);
       assert_eq!(len, output);
    }

    #[test]
    fn test_node_from_n() {
       let input_list = node_1();
       let input_n = 1;
       let output = node_1();
       let result_node = node_from_n(input_n, &input_list);
       assert_eq!(result_node, output);
       let input_list = node_2();
       let input_n = 1;
       let output = node_1();
       let result_node = node_from_n(input_n, &input_list);
       assert_eq!(result_node, output);
       let input_list = node_2();
       let input_n = 2;
       let output = node_2();
       let result_node = node_from_n(input_n, &input_list);
       assert_eq!(result_node, output);
       let input_list = node_3();
       let input_n = 2;
       let output = node_2();
       let result_node = node_from_n(input_n, &input_list);
       assert_eq!(result_node, output);
       let input_list = node_3();
       let input_n = 3;
       let output = node_3();
       let result_node = node_from_n(input_n, &input_list);
       assert_eq!(result_node, output);
       let input_list = node_3();
       let input_n = 2;
       let output = node_2();
       let result_node = node_from_n(input_n, &input_list);
       assert_eq!(result_node, output);
       let input_list = node_3();
       let input_n = 1;
       let output = node_1();
       let result_node = node_from_n(input_n, &input_list);
       assert_eq!(result_node, output);
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
