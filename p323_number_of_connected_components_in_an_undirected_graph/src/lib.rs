struct Solution;
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let graph = edges.into_iter().fold(vec![vec![]; n], |mut g, e| {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y);
            g[y].push(x);
            g
        });
        let mut groups = 0;
        let mut seen = vec![false; n];

        fn dfs(start: usize, seen: &mut Vec<bool>, graph: &Vec<Vec<usize>>) {
            seen[start] = true;
            for &neighbour in graph[start].iter() {
                if !seen[neighbour] {
                    dfs(neighbour, seen, graph);
                }
            }
        }

        for nd in 0..n {
            if !seen[nd] {
                groups += 1;
                dfs(nd, &mut seen, &graph);
            }
        }

        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let edges = vec![vec![0,1],vec![1,2],vec![3,4]];
        let result = Solution::count_components(5, edges);
        assert_eq!(result, 2);
    }

    #[test]
    fn t2() {
        let edges = vec![vec![0,1],vec![1,2],vec![2,3],vec![3,4]];
        let result = Solution::count_components(5, edges);
        assert_eq!(result, 1);
    }

}
