# p557_reverse_words_in_a_string_iii

## two pointers solution

- time complexity of internal loop to reverse the string: O(n/2)
    - the overall time complexity remains O(n)

```
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let (mut i_start_wrd, mut i_end_wrd) = (0, 0);
        let mut string: Vec<char> = s.chars().collect();

        for i_str in 0..string.len() {
            let is_end = i_str == string.len() - 1;
            let space = " ".chars().next().unwrap();
            if string[i_str] == space || is_end {
                i_end_wrd = if is_end { i_str } else { i_str - 1 };
                while i_start_wrd <= i_end_wrd {
                    let start_char = string[i_start_wrd];
                    let end_char = string[i_end_wrd];
                    string[i_end_wrd] = start_char;
                    string[i_start_wrd] = end_char;
                    i_start_wrd += 1;
                    if i_end_wrd == 0 { break; }
                    i_end_wrd -= 1;
                }
                i_start_wrd = i_str + 1;
            }
        }
        string.into_iter().collect()
    }
}
```

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
