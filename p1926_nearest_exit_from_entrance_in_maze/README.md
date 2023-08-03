# p1926_nearest_exit_from_entrance_in_maze

solution with seen as hashmap

```rust
impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let (m,n) = (maze.len(), maze[0].len());
        let entry = (entrance[0], entrance[1]);
        let mut queue = VecDeque::from([(entry, 0)]);
        let mut seen = HashSet::from([entry]);
        let mut ans = i32::MAX;

        while let Some(((i,j),steps)) = queue.pop_front() {
            for (ni_i,nj_i) in [(i+1,j),(i-1,j),(i,j+1),(i,j-1)] {
                let (ni,nj) = (ni_i as usize, nj_i as usize);
                if (ni_i,nj_i) != entry && ni < m && nj < n && maze[ni][nj] != '+' && seen.insert((ni_i,nj_i)){
                    if ni == 0 || nj == 0 || ni == m-1 || nj == n-1 {
                        ans = i32::min(ans, steps+1);
                    }
                    queue.push_back(((ni_i, nj_i), steps+1));
                }
            }
        }

        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
```
