# p20_valid_parentheses_2


```
Runtime 0 ms
Beats 100.00% of users with Rust
```

```rs
pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' if Some(c) != stack.pop() => return false,
            _ => (),
        }
    }
    stack.is_empty()
}
```

---

```
Runtime 1 ms
Beats 49.40% of users with Rust
```

```rs
pub fn is_valid(s: String) -> bool {
    // println!("\n");
    let map = HashMap::from([
        (')', '('),
        ('}', '{'),
        (']', '['),
    ]);
    let mut stack: Vec<char> = vec![s.chars().nth(0).unwrap()];

    for c in s.chars().skip(1) {
        // println!("stack: {:?}", stack);
        // println!("char: {}", c);
        if let Some(opening) = map.get(&c) {
            if !stack.is_empty() &&
                opening == stack.last().unwrap() {
                stack.pop();
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }

    stack.is_empty()
}
```
