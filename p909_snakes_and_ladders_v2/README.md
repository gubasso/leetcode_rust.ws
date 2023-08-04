# p909_snakes_and_ladders_v2

<!-- toc GFM -->

* [Boustrophedon: Find coordinates from node number (and/or node number from coordiantes):](#boustrophedon-find-coordinates-from-node-number-andor-node-number-from-coordiantes)
    - [Simpler/brut force approach: O(n^2) iteration:](#simplerbrut-force-approach-on2-iteration)
    - [Mathematical approach:](#mathematical-approach)
        + [Leetcode solution simpler function:](#leetcode-solution-simpler-function)
        + [My functions:](#my-functions)

<!-- toc -->

## Boustrophedon: Find coordinates from node number (and/or node number from coordiantes):

### Simpler/brut force approach: O(n^2) iteration:

Code ready in python:

```python
n = len(board)
cells = [None] * (n**2 + 1)
label = 1
columns = list(range(0, n))
for row in range(n - 1, -1, -1):
    for column in columns:
        cells[label] = (row, column)
        label += 1
    columns.reverse()
```

Rust rough template (not finalized):

```rust
let n = board.len();
let n_i = n as i32;
let max_node = n.pow(2);
let mut cell = vec![(-1,-1);max_node+1];
let node_label = 1;
for i in (0..n_i).rev() {
    // j needs to be reversed when row is even
    // code not implemented
    for j in 0..n_i {
        cell[node_label] = (i,j);
        node_label += 1;
    }
}
```

### Mathematical approach:

#### Leetcode solution simpler function:

```rust
let boustrophedon = |val: usize| {
    let (i,j) = (val / n, val %n);
    match i%2 {
        0 => (n-1-i, j),    // even flip it
          _ => (n-1-i, n-1-j) // odd don't
    }
};
```

#### My functions:

```rust
fn get_coords_from_node_number(n: i32, node: i32) -> (i32, i32) {
    let m = (node-1)/n;
    let i = n-1-m;
    let row_n_order = n-i;
    let j = if row_n_order % 2 == 0 {
        (n-i)*n - node
    } else {
        (node-1) - m*n
    };
    (i,j)
}

fn get_node_number_from_coords(n: i32, i: i32, j: i32) -> i32 {
    let row_n_order = n-i;
    let base = (n-1-i)*n;
    let diff = if row_n_order % 2 == 0 {
        n-j
    } else {
        j+1
    };
    base + diff
}
```
