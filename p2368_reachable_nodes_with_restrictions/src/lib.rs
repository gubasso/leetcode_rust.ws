struct Solution;
type Edges = Vec<Vec<i32>>;
impl Solution {

    fn dfs(node: usize, seen: &mut Vec<bool>, graph: &Vec<Vec<usize>>) -> i32 {
        let mut count = 1;
        seen[node] = true;
        for &neighbour in graph[node].iter() {
            if !seen[neighbour] {
                count += Self::dfs(neighbour, seen, graph);
            }
        }
        count
    }

    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![];n];

        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            graph[x].push(y);
            graph[y].push(x);
        }
        let mut seen = vec![false; n];
        for r in restricted {
            seen[r as usize] = true;
        }

        seen[0] = true;
        Self::dfs(0, &mut seen, &graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let edges = vec![vec![0,1],vec![1,2],vec![3,1],vec![4,0],vec![0,5],vec![5,6]];
        let result = Solution::reachable_nodes(7, edges, vec![4,5]);
        assert_eq!(result, 4);
    }

    #[test]
    fn t2() {
        let edges = vec![vec![0,1],vec![0,2],vec![0,5],vec![0,4],vec![3,2],vec![6,5]];
        let result = Solution::reachable_nodes(7, edges, vec![4,2,1]);
        assert_eq!(result, 3);
    }

}
