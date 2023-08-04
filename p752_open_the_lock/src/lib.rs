
struct Solution;
use std::collections::{HashSet, VecDeque};
type Code = [usize; 4];
impl Solution {

    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        // println!("\n");
        let deadends: Vec<Code> = deadends.into_iter().map(|code| {
            let mut arr: Code = [0; 4];
            for (i,d) in code.chars().enumerate() {
                arr[i] = d.to_digit(10).unwrap() as usize;
            }
            arr
        }).collect();
        let initial = [0,0,0,0];
        if deadends.contains(&initial) {
            return -1;
        }
        let mut t: Code = [0; 4];
        for (i,d) in target.chars().enumerate() {
            t[i] = d.to_digit(10).unwrap() as usize;
        }
        let mut seen = HashSet::new();
        deadends.iter().for_each(|&code| { seen.insert(code); });
        let inc = |x: usize| ((x as i32 + 1) % 10) as usize;
        let dec = |x: usize| ((x as i32 - 1).rem_euclid(10)) as usize;

        let get_neighbours = |code: Code, seen: &HashSet<Code>| -> Vec<Code> {
            let mut vec = vec![];
            for i in 0..4 {
                for op in ["dec","inc"] {
                    let mut new_code = code;
                    new_code[i] = match op {
                        "dec" => dec(code[i]),
                        _ => inc(code[i]),
                    };
                    if !seen.contains(&new_code) {
                        vec.push(new_code);
                    }
                }
            }
            vec
        };

        let mut queue = VecDeque::from([initial]);
        seen.insert(initial);
        let mut steps = 0;

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let code = queue.pop_front().unwrap();
                if code == t {
                    return steps;
                }
                for neighbour_code in get_neighbours(code, &seen) {
                    // println!("neighbours: {neighbour_code:?}");
                    if seen.insert(neighbour_code) {
                        queue.push_back(neighbour_code);
                    }
                }

            }
            steps += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let deadends = vec!["0201".to_string(),"0101".to_string(),"0102".to_string(),"1212".to_string(),"2002".to_string()];
        let target = "0202".to_string();
        let result = Solution::open_lock(deadends, target);
        assert_eq!(result, 6);
    }

    #[test]
    fn t2() {
        let deadends = vec!["8888".to_string()];
        let target = "0009".to_string();
        let result = Solution::open_lock(deadends, target);
        assert_eq!(result, 1);
    }

    #[test]
    fn t3() {
        let deadends = vec!["8887".to_string(),"8889".to_string(),"8878".to_string(),"8898".to_string(),"8788".to_string(),"8988".to_string(),"7888".to_string(),"9888".to_string()];
        let target = "8888".to_string();
        let result = Solution::open_lock(deadends, target);
        assert_eq!(result, -1);
    }

}
