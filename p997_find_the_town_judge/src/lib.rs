// my first answer
// use std::collections::HashMap;
// use std::collections::HashSet;
// pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
//     let people: Vec<i32> = (1..=n).collect();
//     let mut can_be_not_in_a: HashSet<i32> = Default::default();
//     let mut can_be_in_all_b: HashMap<i32, i32> = Default::default();
//     let mut judge: i32 = 1;
//
//     for i in people.into_iter() { can_be_not_in_a.insert(i); }
//
//     for trust_rule in trust.iter() {
//         let a1: i32 = trust_rule[0];
//         let b1: i32 = trust_rule[1];
//         can_be_not_in_a.remove(&a1);
//         can_be_in_all_b.entry(b1)
//             .and_modify(|e| *e += 1)
//             .or_insert(1);
//     }
//
//     for (k,v) in &can_be_in_all_b {
//         if *v == n-1 {
//             judge = *k;
//             break;
//         }
//     }
//
//     if !can_be_not_in_a.contains(&judge) {
//         judge = -1;
//     }
//
//     if trust.len() == 0 {
//         if n == 1 { judge = 1 }
//         else { judge = -1 };
//     }
//     judge
// }

// my answer after reading the oficial solution
pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {

    let mut people_score: Vec<i32> = vec![0; n as usize];

    if trust.len() < n as usize - 1 {
        return -1;
    }

    for e in trust.iter() {
        people_score[e[0] as usize - 1] -= 1;
        people_score[e[1] as usize - 1] += 1;
    }

    for (i, p) in people_score.iter().enumerate() {
        if *p == n - 1 { return i as i32 + 1 };
    }

    -1
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

    #[test]
    fn ex_5() {
        let n: i32 = 2;
        let trust = vec![];
        let result = find_judge(n, trust);
        assert_eq!(result, -1);
    }

}
