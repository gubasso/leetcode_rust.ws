# p71_simplify_path

Time complexity: O(n)
Space complexity: O(n) (O(2n))

## Functional implementation

Runtime: 2 ms

```
impl Solution {
    pub fn simplify_path(path: String) -> String {
        format!(
            "/{}",
            path.split('/')
            .filter(|&e| !(e.is_empty()) && e != "." )
            .fold(vec![], |mut vec, e| {
                if e == ".." { vec.pop(); }
                else { vec.push(e); }
                vec
            })
            .join("/")
        )
    }
}
```

## Imperative implementation

Runtime: 0 ms

```
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let splits = path.split('/');
        let mut stack = Vec::new();
        for split in splits {
            if split.is_empty() || split == "." { continue; }
            if split == ".." { stack.pop(); continue; }
            stack.push(split)
        }
        format!("/{}", stack.join("/"))
    }
}
```
