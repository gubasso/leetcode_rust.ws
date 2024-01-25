# p125_valid_palindrome

> [125. Valid Palindrome](https://leetcode.com/problems/valid-palindrome/description/)

## My first solution:

- The logic is correct
- Leetcode submission fails because of a big string. It returns a "Time limit exceeded"
- It is because the `s.chars().nth(i).unwrap();` method. Each of these calls iterates over the whole string. It is not efficient. It does not access the char by the index.

```rs
#[cfg(test)]
mod tests {
    struct Solution;
    impl Solution {
        pub fn is_palindrome(s: String) -> bool {
            let last = s.len() - 1;
            let mut i: usize = 0;
            let mut j: usize = last;

            while i < j {
                let mut ci = s.chars().nth(i).unwrap();
                let mut cj = s.chars().nth(j).unwrap();

                if !ci.is_alphanumeric() {
                    i += 1;
                    continue;
                }
                if !cj.is_alphanumeric() {
                    j -= 1;
                    continue;
                }
                if ci.is_uppercase() {
                    for c in ci.to_lowercase() {
                        ci = c;
                    }
                }
                if cj.is_uppercase() {
                    for c in cj.to_lowercase() {
                        cj = c;
                    }
                }
                if ci != cj {
                    return false;
                }
                i += 1;
                j -= 1;
            }

            true
        }
    }
}
```

## Se optimal Rust solution

First, with a "cleaner" code, using `filter_map`:

```
Runtime 5 ms
Beats 10.94% of users with Rust
```

```rs
pub fn is_palindrome(s: String) -> bool {
    let clear_str: String = s.chars()
        .filter_map(|c| c.is_alphanumeric().then(|| c.to_lowercase().collect::<String>()))
        .collect();
    let rev_str: String = clear_str.chars().rev().collect();
    clear_str == rev_str
}
```

Now transforming the string twice: with filter and then to lowercase:

```
Runtime 2 ms
Beats 62.44% of users with Rust
```

```rs
pub fn is_palindrome(s: String) -> bool {
    let mut clear_str : String = s.chars().filter(|x| x.is_alphanumeric()).collect();
    clear_str = clear_str.to_lowercase();
    let str_cpy: String = clear_str.chars().rev().collect();
    clear_str == str_cpy
}
```
