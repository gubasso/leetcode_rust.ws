struct Solution;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::reverse_words("Let's take LeetCode contest".into());
        assert_eq!(result, "s'teL ekat edoCteeL tsetnoc".to_owned());
    }

    #[test]
    fn t2() {
        let result = Solution::reverse_words("God Ding".into());
        assert_eq!(result, "doG gniD".to_owned());
    }

    #[test]
    fn t3() {
        let result = Solution::reverse_words("I love u".into());
        assert_eq!(result, "I evol u".to_owned());
    }

}
