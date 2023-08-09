struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // println!("\n");
        // let alphabet: Vec<u8> = (b'a'..=b'z').collect();
        // let u8_2_i = |u: u8| (u-97) as usize;
        let n = begin_word.len();
        let begin_word = begin_word.into_bytes();
        let end_word = end_word.into_bytes();
        let mut word_list: HashSet<Vec<u8>> = word_list.into_iter().map(|v| v.into_bytes()).collect();
        let mut count = 0;
        if !word_list.contains(&end_word) {
            return count;
        }
        word_list.insert(begin_word.clone());

        let neighbours = |w: &Vec<u8>| -> Vec<Vec<u8>> {
            let mut neighbours = vec![];

            for l in b'a'..=b'z' {
                for i in 0..n {
                    if w[i] == l {
                        continue;
                    }
                    let mut new_word = w.clone();
                    new_word[i] = l;
                    if word_list.contains(&new_word) {
                        neighbours.push(new_word.clone());
                    }
                }
            }

            neighbours
        };

        let mut queue = VecDeque::with_capacity(n);
        queue.push_back(begin_word.clone());
        let mut seen = HashSet::from([begin_word]);

        while !queue.is_empty() {
            count += 1;
            for _ in 0..queue.len() {
                let word = queue.pop_front().unwrap();
                // println!("# word: {:?}, count: {}", word, count);
                if word == end_word {
                    return count;
                }
                for neighbour in neighbours(&word) {
                    if seen.insert(neighbour.clone()) {
                        queue.push_back(neighbour)
                    }
                }

            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec!["hot".to_string(),"dot".to_string(),"dog".to_string(),"lot".to_string(),"log".to_string(),"cog".to_string()];
        let result = Solution::ladder_length(begin_word, end_word, word_list);
        assert_eq!(result, 5);
    }

    #[test]
    fn t2() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec!["hot".to_string(),"dot".to_string(),"dog".to_string(),"lot".to_string(),"log".to_string()];
        let result = Solution::ladder_length(begin_word, end_word, word_list);
        assert_eq!(result, 0);
    }

    #[test]
    fn t3() {
        let begin_word = "hot".to_string();
        let end_word = "dog".to_string();
        let word_list = vec!["hot".to_string(),"dog".to_string()];
        let result = Solution::ladder_length(begin_word, end_word, word_list);
        assert_eq!(result, 0);
    }

}
