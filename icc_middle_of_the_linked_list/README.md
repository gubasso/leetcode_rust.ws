# icc_middle_of_the_linked_list

## Fast/Slow Pointers approach

- time complexity: O(n/2) -> O(n)
    - it is proportional to the number of inputs
- space complexity: O(1)
    - just two pointers are declared, independent from the number of inputs

### Naive implementation (basic Rust code)

```
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = &head;
    let mut slow = &head;

    loop {
        if let Some(fast_node) = fast {
            fast = &fast_node.next;
            if let Some(fast_node) = fast {
                fast = &fast_node.next;
            } else {
                break;
            }
        } else {
            break;
        };
        slow = if let Some(slow_node) = slow { &slow_node.next } else { &None };
    }

    slow.clone()
}
```

### Best implementation with elegant resources from Rust language

- https://leetcode.com/problems/middle-of-the-linked-list/solutions/3334252/java-rust-python-two-pointers-solution-with-clear-explanation-and-real-life-application/

```
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;
    
    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    return slow.clone();
}
```

