# icc_find_players_with_zero_or_one_losses

## Counting with Array Approach (Counting Sort)

- time complexity: O(n + k)
- space complexity: O(k)

```
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut lost_count: Vec<i32> = vec![-1; 100001];
        let mut ans = vec![vec![], vec![]];

        for m in matches {
            let winner = m[0] as usize;
            let loser = m[1] as usize;
            lost_count[winner] += if lost_count[winner] == -1 { 1 } else { 0 };
            lost_count[loser] += if lost_count[loser] == -1 { 2 } else { 1 };
        }

        for (player, losses) in lost_count.iter().enumerate() {
            if *losses == 0 { ans[0].push(player as i32) };
            if *losses == 1 { ans[1].push(player as i32) };
        }

        ans
    }
}
```



## Hash Map Approach

- time complexity: O(n.log n)
    - `log n` because of the sorting at the end
- space complexity: O(n)

```
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut losts: HashMap<i32, i32> = HashMap::new();
        let mut ans = vec![vec![], vec![]];

        for m in matches {
            losts.entry(m[1]).and_modify(|c| *c += 1).or_insert(1);
            losts.entry(m[0]).or_insert(0);
        }

        for (k, v) in losts {
            if v == 0 { ans[0].push(k) };
            if v == 1 { ans[1].push(k) };
        }

        ans[0].sort();
        ans[1].sort();
        ans
    }
}

```

