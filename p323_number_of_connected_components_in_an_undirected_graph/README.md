# p323_number_of_connected_components_in_an_undirected_graph


DFS Recursive

```rust
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let graph = edges.into_iter().fold(vec![vec![]; n], |mut g, e| {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y);
            g[y].push(x);
            g
        });
        let mut groups = 0;
        let mut seen = vec![false; n];

        fn dfs(start: usize, seen: &mut Vec<bool>, graph: &Vec<Vec<usize>>) {
            seen[start] = true;
            for &neighbour in graph[start].iter() {
                if !seen[neighbour] {
                    dfs(neighbour, seen, graph);
                }
            }
        }

        for nd in 0..n {
            if !seen[nd] {
                groups += 1;
                dfs(nd, &mut seen, &graph);
            }
        }

        groups
    }
}
```

DFS iterative

```rust
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let graph = edges.into_iter().fold(vec![vec![]; n], |mut g, e| {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y);
            g[y].push(x);
            g
        });
        let mut groups = 0;
        let mut seen = vec![false; n];

        let dfs = |start: usize, seen: &mut Vec<bool>| {
            let mut stack = vec![start];

            while let Some(node) = stack.pop() {
                seen[node] = true;
                for &neighbour in graph[node].iter() {
                    if !seen[neighbour] {
                        stack.push(neighbour);
                    }
                }
            }

        };

        for nd in 0..n {
            if !seen[nd] {
                groups += 1;
                dfs(nd, &mut seen);
            }
        }

        groups
    }
}
```
