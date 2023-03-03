pub fn str_str(haystack: String, needle: String) -> i32 {

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let result = str_str("sadbutsad".to_string(), "sad".to_string());
        assert_eq!(result, 4);
    }
}
