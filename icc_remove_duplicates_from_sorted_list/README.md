# icc_remove_duplicates_from_sorted_list

## Wrong approach: with Set

This approach does not consider the native linked list manipulation, which is the porpose of this problem.

```
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut set: HashSet<i32> = HashSet::new();
    let mut dummy = &head;
    while let Some(node) = dummy {
        set.insert(node.val);
        dummy = &node.next;
    }
    let mut vec: Vec<i32> = set.into_iter().collect();
    vec.sort();
    list_from_vec(&vec)
}
```

## Correct approach: only linked list / node techniques

- https://leetcode.com/problems/remove-duplicates-from-sorted-list/solutions/344741/my-idiomatic-solution-rust/

```
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
```
