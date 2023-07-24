# p200_number_of_islands


Solution with manual handling the usize overflow:

```rust
impl Solution {

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let row = grid.len();
        let col = grid[0].len();
        let diffs = [0,1,0,-1,0];
        let mut n_islands = 0;

        let bfs = |node: (usize, usize), grid: &mut Vec<Vec<char>>| {
            let mut queue = VecDeque::from([node]);
            while let Some((r,c)) = queue.pop_front() {
                for d in diffs.windows(2) {
                    let mut tup: [Option<usize>; 2] = [None; 2];
                    let r_i = r as i32 + d[0];
                    let c_i = c as i32 + d[1];
                    for (i, n) in [r_i, c_i].into_iter().enumerate() {
                        tup[i] = if n >= 0 {
                            Some(n as usize)
                        } else {
                            None
                        }
                    }

                    if let [Some(r), Some(c)] = tup {
                        if r < row && c < col && grid[r][c] == '1' {
                            grid[r][c] = '0';
                            queue.push_back((r,c));
                        }
                    }

                }
            }
        };

        for r in 0..row {
            for c in 0..col {
                if grid[r][c] == '1' {
                    n_islands += 1;
                    bfs((r,c), &mut grid);
                }
            }
        }

        n_islands
    }

}
```



Solution with usize bounded by `checked_add_signed` function:

```rust
impl Solution {

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let row = grid.len();
        let col = grid[0].len();
        let diffs = [0,1,0,-1,0];
        let mut n_islands = 0;

        let bfs = |node: (usize, usize), grid: &mut Vec<Vec<char>>| {
            let mut queue = VecDeque::from([node]);
            while let Some((r,c)) = queue.pop_front() {
                for d in diffs.windows(2) {
                    let n_r = r.checked_add_signed(d[0]);
                    let n_c = c.checked_add_signed(d[1]);
                    if let (Some(r), Some(c)) = (n_r, n_c) {
                        if r < row && c < col && grid[r][c] == '1' {
                            grid[r][c] = '0';
                            queue.push_back((r,c));
                        }
                    }
                }
            }
        };

        for r in 0..row {
            for c in 0..col {
                if grid[r][c] == '1' {
                    n_islands += 1;
                    bfs((r,c), &mut grid);
                }
            }
        }

        n_islands
    }

}
```
