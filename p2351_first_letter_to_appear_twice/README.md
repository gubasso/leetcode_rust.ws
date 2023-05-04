# p2351_first_letter_to_appear_twice

## Hashmap solution

```
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut chars_freq = HashMap::new();

        for ch in s.chars() {
            chars_freq.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
            if chars_freq.get(&ch) == Some(&2) { return ch; }
        }

        ' '
    }
}
```

## HashSet solution

**functional style**
```
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut chars = HashSet::new();
        s.chars().skip_while(|&c| chars.insert(c)).next().unwrap()
    }
}
```

**for loop**
```
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut chars = HashSet::new();
        for ch in s.chars() {
            if !chars.insert(ch) { return ch; }
        }
        ' '
    }
}
```

