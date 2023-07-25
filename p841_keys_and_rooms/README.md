# p841_keys_and_rooms

dfs iterative with count

```rust
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut stack = Vec::with_capacity(n);
        let mut seen = vec![false; n];
        let mut count = 1;
        stack.push(0);
        seen[0] = true;

        while let Some(room) = stack.pop() {
            for key in rooms[room].iter().map(|k| *k as usize) {
                if !seen[key] {
                    count += 1;
                    if count == n {
                        return true;
                    }
                    seen[key] = true;
                    stack.push(key)
                }
            }
        }

        false
    }
}
```

dfs recursive with count

```rust
impl Solution {

    fn dfs(room: i32, rooms: &Vec<Vec<i32>>, seen: &mut Vec<bool>) -> i32 {
        let mut count = 1;
        for key in rooms[room as usize].iter() {
            let k = *key as usize;
            if !seen[k] {
                seen[k] = true;
                count += Self::dfs(*key, rooms, seen);
            }
        }
        count
    }

    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut seen = vec![false; rooms.len()];
        seen[0] = true;
        Self::dfs(0, &rooms, &mut seen) == rooms.len() as i32
    }
}
```


Dfs recursive solution checking if all seen are true.

```rust
impl Solution {

    fn dfs(room: i32, rooms: &Vec<Vec<i32>>, seen: &mut Vec<bool>) {
        for key in rooms[room as usize].iter() {
            let k = *key as usize;
            if !seen[k] {
                seen[k] = true;
                Self::dfs(*key, rooms, seen);
            }
        }
    }

    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut seen = vec![false; rooms.len()];
        seen[0] = true;
        Self::dfs(0, &rooms, &mut seen);
        !seen.into_iter().any(|s| !s)
    }
}
```

