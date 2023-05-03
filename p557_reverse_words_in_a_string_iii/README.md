# p557_reverse_words_in_a_string_iii

## two pointers solution


## normal solution:

- time complexity: O(n + n) = O(n)
- space complexity: O(1)

```
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let (mut i, mut j) = (0, 0);
        let string: Vec<char> = s.chars().collect();
        let mut new_string: Vec<char> = Vec::new();

        for sp in 0..string.len() {
            let is_end = sp + 1 == string.len();
            let space = " ".chars().next().unwrap();
            if string[sp] == space || is_end {
                if is_end { j = sp } else { j = sp - 1 };

                for rp in (i..=j).rev() {
                    new_string.push(string[rp]);
                }
                if !is_end { new_string.push(space) }
                i = sp + 1;
            }
        }
        new_string.into_iter().collect()
    }
}
```
