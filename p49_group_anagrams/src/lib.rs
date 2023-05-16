use std::collections::HashMap;

struct Solution;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::group_anagrams(vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()]);
        assert_eq!(result, vec![vec!["bat".to_string()],vec!["nat".to_string(),"tan".to_string()],vec!["ate".to_string(),"eat".to_string(),"tea".to_string()]]);
    }

    #[test]
    fn t2() {
        let result = Solution::group_anagrams(vec!["".to_string()]);
        assert_eq!(result, vec![vec!["".to_string()]]);
    }

    #[test]
    fn t3() {
        let result = Solution::group_anagrams(vec!["a".to_string()]);
        assert_eq!(result, vec![vec!["a".to_string()]]);
    }

}
