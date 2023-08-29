struct Solution;

use std::collections::HashMap;
impl Solution {

    fn backtrack(map: &HashMap<i32, Vec<char>>,
                 digits: &Vec<i32>, i: usize,
                 ans: &mut Vec<String>,
                 letters: &mut Vec<char>) {
        if digits.len() == letters.len() {
            ans.push(letters.iter().collect());
            return;
        }

        for l in map.get(&digits[i]).unwrap() {
            letters.push(*l);
            Self::backtrack(map, digits, i+1, ans, letters);
            letters.pop();
        }

    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let v = [(2, vec!['a', 'b', 'c']), (3, vec!['d', 'e', 'f']), (4, vec!['g', 'h', 'i']), (5, vec!['j', 'k', 'l']), (6, vec!['m', 'n', 'o']), (7, vec!['p', 'q', 'r', 's']), (8, vec!['t', 'u', 'v']), (9, vec!['w', 'x', 'y', 'z'])];
        let map: HashMap<i32, Vec<char>> = HashMap::from(v);
        let digits = digits.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        let mut ans = vec![];
        let mut letters = vec![];
        Self::backtrack(&map, &digits, 0, &mut ans, &mut letters);
        ans
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let digits = "23".to_string();
        let out = vec!["ad".to_string(),"ae".to_string(),"af".to_string(),"bd".to_string(),"be".to_string(),"bf".to_string(),"cd".to_string(),"ce".to_string(),"cf".to_string()];
        let result = Solution::letter_combinations(digits);
        assert_eq!(result, out);
    }

    #[test]
    fn t2() {
        let digits = "".to_string();
        let out: Vec<String> = vec![];
        let result = Solution::letter_combinations(digits);
        assert_eq!(result, out);
    }

    #[test]
    fn t3() {
        let digits = "2".to_string();
        let out = vec!["a".to_string(),"b".to_string(),"c".to_string()];
        let result = Solution::letter_combinations(digits);
        assert_eq!(result, out);
    }

}
