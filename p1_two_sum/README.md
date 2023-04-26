# p1_two_sum

## Solution: hashmap

- Time complexity: O(n)
- Space complexity: O(n)

```
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, curr) in nums.iter().enumerate() {
            let diff: i32 = target - curr;

            if let Some(idx) = map.get(&diff) {
                return vec![*idx, i as i32];
            } else {
                map.insert(*curr, i as i32);
            };

        }
        vec![]
    }
}
```

## Solution: brute force 

- Time complexity: O(n^2)
- Space complexity: O(1)

```
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut coord: Vec<i32> = Vec::new();
        for (left, _e) in nums.iter().enumerate() {
            for right in left+1..nums.len() {
                if nums[left] + nums[right] == target {
                    coord = vec![left as i32, right as i32];
                    return coord;
                }
            }
        }
        coord
    }
}
```
