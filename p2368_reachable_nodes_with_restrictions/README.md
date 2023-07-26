# p2368_reachable_nodes_with_restrictions

dfs recursive

```rust
type Edges = Vec<Vec<i32>>;
impl Solution {

    fn dfs(node: usize, seen: &mut Vec<bool>, graph: &Vec<Vec<usize>>) -> i32 {
        let mut count = 1;
        seen[node] = true;
        for &neighbour in graph[node].iter() {
            if !seen[neighbour] {
                count += Self::dfs(neighbour, seen, graph);
            }
        }
        count
    }

    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![];n];

        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            graph[x].push(y);
            graph[y].push(x);
        }
        let mut seen = vec![false; n];
        for r in restricted {
            seen[r as usize] = true;
        }

        seen[0] = true;
        Self::dfs(0, &mut seen, &graph)
    }
}
```

dfs iterative

```rust
type Edges = Vec<Vec<i32>>;
impl Solution {

    fn dfs(node: usize, seen: &mut Vec<bool>, graph: &Vec<Vec<usize>>) -> i32 {
        let mut stack = vec![node];
        let mut count = 0;
        seen[node] = true;

        while let Some(nd) = stack.pop() {
            count += 1;
            for &child in graph[nd].iter() {
                if !seen[child] {
                    seen[nd] = true;
                    stack.push(child);
                }
            }
        }

        count
    }

    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![];n];

        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            graph[x].push(y);
            graph[y].push(x);
        }
        let mut seen = vec![false; n];
        for r in restricted {
            seen[r as usize] = true;
        }


        Self::dfs(0, &mut seen, &graph)
    }
}
```
