fn compute_lps_array(needle: &String, m: usize) -> Vec<usize> {
    let mut lps: Vec<usize> = vec![0; m];
    let mut len: usize = 0;
    let mut i: usize = 1;
    lps[0] = 0;

    while i < m {
        if needle.chars().nth(i) == needle.chars().nth(len) {
            lps[i] = len + 1;
            i += 1;
            len += 1;
        } else if len != 0 {
            len = lps[len-1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }

    lps
}


pub fn str_str(haystack: String, needle: String) -> i32 {
    let n = haystack.len();
    let m = needle.len();
    if m > n { return -1 };
    if n < 1 { return -1 };
    if m >= 10000 { return -1 };
    let lps = compute_lps_array(&needle, m);
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < n {
        if haystack.chars().nth(i) == needle.chars().nth(j) {
            i += 1;
            j += 1;
        } else if j != 0 {
            j = lps[j-1];
        } else {
            i += 1;
        }
        if j == m {
            return i as i32 - j as i32;
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

    #[test]
    fn case_3() {
        let result = str_str("a".to_string(), "a".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn case_4() {
        let result = str_str("aaa".to_string(), "aaa".to_string());
        assert_eq!(result, 0);
    }

}
