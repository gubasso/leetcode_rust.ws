struct Solution;
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {

        // println!("\n");
        let mut graph = HashMap::new();
        let n = queries.len();
        let mut ans = vec![-1.0; n];

        for (i, eq) in equations.into_iter().enumerate() {
            let (num, den) = (eq[0].clone(), eq[1].clone());
            let val = values[i];
            let inv = 1.0/val;
            graph.entry(num.clone()).and_modify(|entry: &mut HashMap<String,f64>| {
                entry.insert(den.clone(), val);
            }).or_insert(HashMap::from([(den.clone(), val)]));
            graph.entry(den).and_modify(|entry: &mut HashMap<String,f64>| {
                entry.insert(num.clone(), inv);
            }).or_insert(HashMap::from([(num.clone(), inv)]));
        }

        let calculate = |start: String, end: String| -> f64 {

            if !graph.contains_key(&start) {
                return -1.0;
            }
            let mut stack = Vec::from([(start.clone(), 1.0)]);
            let mut seen =  HashSet::from([start.clone()]);

            while let Some((entry, ratio)) = stack.pop() {
                if entry == end {
                    return ratio;
                }
                for nentry in graph.get(&entry).unwrap().keys() {
                    if seen.insert(nentry.clone()) {
                        stack.push((nentry.clone(), ratio * graph[&entry][nentry]));
                    }

                }
            }

            -1.0
        };

        // println!("{:#?}", graph);
        calculate("b".to_string(), "b".to_string());
        for (i, q) in queries.into_iter().enumerate() {
            let (num,den) = (q[0].clone(),q[1].clone());
            ans[i] = calculate(num,den);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let equations = vec![vec!["a".to_string(),"b".to_string()],vec!["b".to_string(),"c".to_string()]];
        let values = vec![2.0,3.0];
        let queries = vec![vec!["a".to_string(),"c".to_string()],vec!["b".to_string(),"a".to_string()],vec!["a".to_string(),"e".to_string()],vec!["a".to_string(),"a".to_string()],vec!["x".to_string(),"x".to_string()]];
        let result = Solution::calc_equation(equations, values, queries);
        assert_eq!(result, vec![6.00000,0.50000,-1.00000,1.00000,-1.00000]);
    }

    #[test]
    fn t2() {
        let equations = vec![vec!["a".to_string(),"b".to_string()],vec!["b".to_string(),"c".to_string()],vec!["bc".to_string(),"cd".to_string()]];
        let values = vec![1.5,2.5,5.0];
        let queries = vec![vec!["a".to_string(),"c".to_string()],vec!["c".to_string(),"b".to_string()],vec!["bc".to_string(),"cd".to_string()],vec!["cd".to_string(),"bc".to_string()]];
        let result = Solution::calc_equation(equations, values, queries);
        assert_eq!(result, vec![3.75000,0.40000,5.00000,0.20000]);
    }

    #[test]
    fn t3() {
        let equations = vec![vec!["a".to_string(),"b".to_string()]];
        let values = vec![0.5];
        let queries = vec![vec!["a".to_string(),"b".to_string()],vec!["b".to_string(),"a".to_string()],vec!["a".to_string(),"c".to_string()],vec!["x".to_string(),"y".to_string()]];
        let result = Solution::calc_equation(equations, values, queries);
        assert_eq!(result, vec![0.50000,2.00000,-1.00000,-1.00000]);
    }

}
