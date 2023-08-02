# p542_01_matrix

Solution using the map/grid with a `-1` marker for the nodes seen

```rust

```

Solution with seen hashmap

```rust
impl Solution {
    fn get_valid_neighbours(i: i32, j: i32, m: i32, n: i32) -> Vec<(usize, usize)> {
        let mut coords = vec![];
        let is_valid = |i,j| {
            i >= 0 && j >= 0 && i < m && j < n
        };

        for di in -1..=1 {
            for dj in -1..=1 {
                if (di,dj) == (0,0) || (di != 0 && dj != 0) {
                    continue;
                }
                let r = i + di;
                let c = j + dj;
                if is_valid(r,c) {
                    coords.push((r as usize,c as usize));
                }
            }
        }

        coords
    }

    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m,n) = (mat.len(), mat[0].len());
        let mut queue = VecDeque::with_capacity(m*n);
        let mut seen = HashSet::new();

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    queue.push_back((i,j,1));
                    seen.insert((i,j));
                }
            }
        }

        while let Some((i,j,steps)) = queue.pop_front() {
            for (ni, nj) in Self::get_valid_neighbours(i as i32, j as i32, m as i32, n as i32) {
                if seen.insert((ni,nj)) {
                    mat[ni][nj] = steps;
                    queue.push_back((ni, nj, steps + 1));
                }
            }
        }

        mat
    }
}
```
