# p547_number_of_provinces

Not creating any map/graph adjacent list. And running with bfs:

runtime: 0ms

```rust
impl Solution {
    fn bfs(start: usize, is_connected: &Vec<Vec<i32>>, seen: &mut Vec<bool>) {
        let len = is_connected.len();
        let mut queue = VecDeque::with_capacity(len);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            for j in 0..len {
                if !seen[j] && is_connected[node][j] > 0 {
                    queue.push_back(j);
                    seen[j] = true;
                }
            }
        }

    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let len = is_connected.len();
        let mut groups = 0;
        let mut seen = vec![false; len];

        for i in 0..len {
            if seen[i] { continue; }
            groups += 1;
            seen[i] = true;
            Self::bfs(i, &is_connected, &mut seen);
        }

        groups
    }
}
```

Creating map/graph adjacent list first + dfs:

```rust
impl Solution {

    fn dfs(start: usize, map: &Vec<Vec<usize>>,seen: &mut [bool]) {
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            for neighbour in map[node].iter() {
                if !seen[*neighbour] {
                    seen[*neighbour] = true;
                    stack.push(*neighbour);
                }
            }
        }
    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let len = is_connected.len();
        let mut map: Vec<Vec<usize>> = vec![vec![]; len];
        let mut seen: Vec<bool> = vec![false; len];
        let mut ans = 0;

        // build map graph
        for i in 0..len {
            for j in (i+1)..len {
                if is_connected[i][j] == 1 {
                    map[i].push(j);
                    map[j].push(i);
                }
            }
        }

        for i in 0..len {
            if !seen[i] {
                ans += 1;
                seen[i] = true;
                Self::dfs(i, &map, &mut seen);
            }
        }


        ans
    }
}
```

