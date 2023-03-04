pub fn str_str(haystack: String, needle: String) -> i32 {
    let first_letter: char = needle.chars().next().unwrap();
    for (hi, hc) in haystack.chars().enumerate() {
        if hc == first_letter {
            let mut is_equal: bool = false;
            let mut loop_counter: usize = hi;
            for nc in needle.chars() {
                is_equal = if let Some(hc_char) = haystack.chars().nth(loop_counter) {
                    nc == hc_char
                } else { false };
                if !is_equal { break; };
                loop_counter += 1;
            }
            if is_equal {
                return hi as i32;
            };
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let result = str_str("sadbutsad".to_string(), "sad".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn case_2() {
        let result = str_str("leetcode".to_string(), "leeto".to_string());
        assert_eq!(result, -1);
    }

}
