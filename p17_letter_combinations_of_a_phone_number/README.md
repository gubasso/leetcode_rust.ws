# p17_letter_combinations_of_a_phone_number

```rust
impl Solution {

    fn backtrack(n: usize, map: &HashMap<i32, Vec<char>>,
                 digits: &Vec<i32>, i: usize,
                 ans: &mut Vec<String>,
                 mut letters: Vec<char>) {

        if n == letters.len() {
            ans.push(letters.iter().collect());
            return;
        }

        for l in map.get(&digits[i]).unwrap() {
            letters.push(l.clone());
            Self::backtrack(n, map, digits, i+1, ans, letters.to_vec());
            letters.pop();
        }

    }

    pub fn letter_combinations(digits: String) -> Vec<String> {

        if digits.is_empty() {
            return vec![];
        }

        let v = [(2, vec!['a', 'b', 'c']), (3, vec!['d', 'e', 'f']), (4, vec!['g', 'h', 'i']), (5, vec!['j', 'k', 'l']), (6, vec!['m', 'n', 'o']), (7, vec!['p', 'q', 'r', 's']), (8, vec!['t', 'u', 'v']), (9, vec!['w', 'x', 'y', 'z'])];
        let map: HashMap<i32, Vec<char>> = HashMap::from(v);
        let digits = digits.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
        let n = digits.len();
        let mut ans = vec![];
        Self::backtrack(n, &map, &digits, 0, &mut ans, vec![]);


        ans
    }
}
```
