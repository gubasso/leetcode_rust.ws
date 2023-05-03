struct Solution;
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut string: Vec<char> = s.chars().collect();
        let (mut i, mut j) = (0, string.len() - 1);

        while i < j {
            if !string[i].is_alphabetic() {
                i += 1;
                continue;
            }
            if !string[j].is_alphabetic() {
                j -= 1;
                continue;
            }
            let left_char = string[i];
            let right_char = string[j];
            string[i] = right_char;
            string[j] = left_char;
            i += 1;
            j -= 1;
        }

        string.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::reverse_only_letters("ab-cd".into());
        assert_eq!(result, "dc-ba".to_owned());
    }

    #[test]
    fn t2() {
        let result = Solution::reverse_only_letters("a-bC-dEf-ghIj".into());
        assert_eq!(result, "j-Ih-gfE-dCba".to_owned());
    }

    #[test]
    fn t3() {
        let result = Solution::reverse_only_letters("Test1ng-Leet=code-Q!".into());
        assert_eq!(result, "Qedo1ct-eeLg=ntse-T!".to_owned());
    }

}
