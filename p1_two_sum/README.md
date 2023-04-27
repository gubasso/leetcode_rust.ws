# p1_two_sum

## Solution: two pointers

Explanation:
We make a copy of the existing array.
The reason for the copy is when we find the left/right pointers, 
we've found pointers that only apply to the sorted array. The problem asks for the
pointers in the original array, and we need to use these pointers to 
find their corresponding indices in int[] nums.

We use the left/right pointers to narrow down our two pointers to see if anything matches. 
Since the copy array is sorted, we know that if our sum is less than
the target, we need to increase the left pointer and see if we can find a larger 
number that adds up to target.
Otherwise, we know the number at the right pointer is too large, 
and we need to find a smaller number, so we reduce the right pointer.

Once we have the pointers, we need to use them to find the original values.
Otherwise, if we have found nothing, we will end up eventually returning an empty result.

In the scenario where we have two valid left/right pointers, we need to loop
through the original array and find the indices corresponding to the numbers we found. 
To find the left pointer, we start at 0 and go through the array, matching the first number
we see with the number that corresponds to our left pointer.

We do something similar with the right pointer, but the trick is to loop
through the array backwards, to find the right pointer first. This is because we may have the 
same number that sums up to the target, and we need to return them in order. 
You could also rewrite this in another way where we have a "is First written "flag or something. not a big deal.

Time complexity: O(n log n)
    - log n because of the initial sorting

```
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left: usize = 0;
    let mut right: usize = nums.len()-1;
    let mut ordered: Vec<(usize, &i32)> = nums.iter().enumerate().collect();
    ordered.sort_by(|a,b| a.1.cmp(&b.1));

    for _e in &ordered {
        let sum = ordered[left].1 + ordered[right].1;
        if sum == target {
            return vec![ordered[left].0 as i32, ordered[right].0 as i32];
        } else if sum < target {
            left += 1;
        } else if sum > target {
            right -= 1;
        }
    }

    vec![]
}
```

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
