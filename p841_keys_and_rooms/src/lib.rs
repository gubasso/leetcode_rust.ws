struct Solution;
impl Solution {

    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut stack = Vec::with_capacity(n);
        let mut seen = vec![false; n];
        let mut count = 1;
        stack.push(0);
        seen[0] = true;

        while let Some(room) = stack.pop() {
            for key in rooms[room].iter().map(|k| *k as usize) {
                if !seen[key] {
                    count += 1;
                    if count == n {
                        return true;
                    }
                    seen[key] = true;
                    stack.push(key)
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let input = vec![vec![1],vec![2],vec![3],vec![]];
        let result = Solution::can_visit_all_rooms(input);
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let input = vec![vec![1,3],vec![3,0,1],vec![2],vec![0]];
        let result = Solution::can_visit_all_rooms(input);
        assert_eq!(result, false);
    }

}
