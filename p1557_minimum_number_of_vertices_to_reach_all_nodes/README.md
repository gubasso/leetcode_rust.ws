# p1557_minimum_number_of_vertices_to_reach_all_nodes

optimal functional solution. Runtime 30ms.

```rust
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        edges.into_iter().fold((0..n).collect::<Vec<i32>>(), |mut vec, edge| {
            vec[edge[1] as usize] = -1;
            vec
        }).into_iter().filter(|&e| e != -1).collect()
    }
}
```

first easy to read solution. Runtime 39ms

```rust
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut indegrees = vec![0; n];
        for e in edges {
            indegrees[e[1] as usize] += 1;
        }

        indegrees.into_iter().enumerate().filter_map(|(i, j)| {
            return if j == 0 {
                Some(i as i32)
            } else {
                None
            };
        }).collect()
    }
}
```
