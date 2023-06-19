# p20_valid_parentheses

- Time complexity: O(n)
- Space complexity: O(n)
    - because of the stack space

## First simple solution

```
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let map = HashMap::from([
                                (')', '('),
                                (']', '['),
                                ('}', '{'),
        ]);

        for c in s.chars() {
            let counterpart = map.get(&c);
            if let Some(closing) = counterpart {
                if let Some(last_elem) = stack.last() {
                    if *last_elem == *closing {
                        stack.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }

            } else {
                stack.push(c);
            }
        }

        stack.is_empty()
    }
}
```

## More concite, idiomatic, elegant solution

- https://leetcode.com/problems/valid-parentheses/solutions/2715693/match-brackets-rust-solution/

```
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut brackets = Vec::new();

        for bracket in s.chars() {
            match bracket {
                '{' => brackets.push('}'),
                '(' => brackets.push(')'),
                '[' => brackets.push(']'),

                closing => if Some(closing) != brackets.pop() { 
                    return false 
                }
            }
        }
        brackets.is_empty()
    }
}
```

