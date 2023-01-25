use std::collections::HashMap;
use std::collections::HashSet;

pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let people: Vec<i32> = (1..=n).collect();
    let mut can_be_not_in_a: HashSet<i32> = Default::default();
    let mut can_be_in_all_b: HashMap<i32, i32> = Default::default();
    let mut judge: i32 = 1;

    for i in people.into_iter() { can_be_not_in_a.insert(i); }

    for trust_rule in trust.iter() {
        let a1: i32 = trust_rule[0];
        let b1: i32 = trust_rule[1];
        can_be_not_in_a.remove(&a1);
        can_be_in_all_b.entry(b1)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for (k,v) in &can_be_in_all_b {
        if *v == n-1 {
            judge = *k;
            break;
        }
    }

    if !can_be_not_in_a.contains(&judge) {
        if trust.len() != 0 { judge = -1; }
    }
    judge
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_1() {
        let n: i32 = 2;
        let trust = vec![vec![1,2]];
        let result = find_judge(n, trust);
        assert_eq!(result, 2);
    }

    #[test]
    fn ex_2() {
        let n: i32 = 3;
        let trust = vec![vec![1,3], vec![2,3]];
        let result = find_judge(n, trust);
        assert_eq!(result, 3);
    }

    #[test]
    fn ex_3() {
        let n: i32 = 3;
        let trust = vec![vec![1,3],vec![2,3],vec![3,1]];
        let result = find_judge(n, trust);
        assert_eq!(result, -1);
    }

    #[test]
    fn ex_4() {
        let n: i32 = 1;
        let trust = vec![];
        let result = find_judge(n, trust);
        assert_eq!(result, 1);
    }

}
