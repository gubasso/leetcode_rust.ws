# p49_group_anagrams

<!-- toc GFM -->

* [Approach 1: Categorize by Sorted String](#approach-1-categorize-by-sorted-string)
* [Approach 2: Categorize by Count](#approach-2-categorize-by-count)
    - [functional implementation](#functional-implementation)
    - [Normal / Naive implementation](#normal--naive-implementation)

<!-- toc -->

## Approach 1: Categorize by Sorted String

- time complexity: O(N*K*logK)
    - iterate over `strs`: O(n)
    - sort each string: O(K*log*K)
- space complexity: O(n*k)
    - answer array has N strings of length K

```
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs {
            let mut sorted: Vec<char> = str.chars().collect();
            sorted.sort_by(|a, b| a.cmp(b));
            let sorted: String = sorted.into_iter().collect();
            map.entry(sorted.clone())
                .and_modify(|vec| {
                    vec.push(str.clone());
                    vec.sort();
                })
                .or_insert(vec![str.clone()]);
        }

        let mut ans = vec![];

        for (_, v) in map { ans.push(v); }

        ans.sort_by( |a, b| a.len().cmp(&b.len()) );
        ans

    }
}
```

## Approach 2: Categorize by Count

- time complexity: O(N*K)
- space complexity: O(N*K)

### functional implementation

- from: https://leetcode.com/problems/group-anagrams/solutions/2751413/rust-hashmap-functional-style-with-comments/


"Strings are grouped by the histogram of letter frequencies. Use this as the key in a hash map, collecting the strings in the map values. I had hopes of making this a one-liner, but have not figured out a way to get around that we have to return the modified histogram/map in the fold body. :)"

"Implementation notes:

- N_LETTERS instead of the magic value 26.
- strs.into_iter to take ownership of the provided strings and move them into the map.
- or_default for the most concise insertion of an empty vector if this is the first time we encounter a key.
- into_values to directly map the values to the output iterator, instead of into_iter().map(...), and to take ownership of the hash map values => the string instances we get as input are the same instances that we return in the output.
"

```
use std::collections::HashMap;

const N_LETTERS: usize = (b'z' - b'a' + 1) as _;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter().fold(HashMap::<[u8; N_LETTERS], Vec<String>>::new(), |mut map, s| {
            let freqs = s.bytes().map(|b| (b - b'a') as usize).fold([0; N_LETTERS], |mut freqs, bin| {
                freqs[bin] += 1;
                freqs
            });
            map.entry(freqs).or_default().push(s);
            map
        }).into_values().collect()
    }
}
```

### Normal / Naive implementation


```
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for str in &strs {
            let mut chars: [u32; 26] = [0; 26];
            for c in str.chars() {
                chars[c as usize - 97] += 1;
            }
            let mut key: String = String::new();
            for count in chars {
                key.push_str(&count.to_string());
                key.push_str("#");
            }
            map.entry(key)
                .and_modify(|vec| { vec.push(str.clone()) })
                .or_insert(vec![str.clone()]);
        }

        let mut ans: Vec<Vec<String>> = Vec::new();
        for v in map.values_mut() {
            v.sort();
            ans.push(v.to_vec());
        }

        ans.sort_by( |a, b| a.len().cmp(&b.len()) );

        ans
    }
}
```

