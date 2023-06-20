struct Solution;
impl Solution {
    pub fn simplify_path(path: String) -> String {
        format!(
            "/{}",
            path.split('/')
            .filter(|&e| !(e.is_empty()) && e != "." )
            .fold(vec![], |mut vec, e| {
                if e == ".." { vec.pop(); }
                else { vec.push(e); }
                vec
            })
            .join("/")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::simplify_path("/home/".to_string());
        assert_eq!(result, "/home".to_string());
    }

    #[test]
    fn t2() {
        let result = Solution::simplify_path("/../".to_string());
        assert_eq!(result, "/".to_string());
    }

    #[test]
    fn t3() {
        let result = Solution::simplify_path("/home//foo/".to_string());
        assert_eq!(result, "/home/foo".to_string());
    }

}
