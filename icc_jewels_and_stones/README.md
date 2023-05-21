# icc_jewels_and_stones

## Hashset approach

- time complexity: O(n)
- space complexity: O(k)

### set + fold:

```
pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut set: HashSet<char> = HashSet::from_iter(jewels.chars());
    stones.chars().fold(0, |acc, s| {
        if set.contains(&s) { acc + 1 } else { acc + 0 }
    })
}
```

### set + filter + count

```
pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let jewels_set = jewels.chars().collect::<HashSet<char>>();
    stones.chars().filter(|c| jewels_set.contains(c)).count() as i32
}
```


