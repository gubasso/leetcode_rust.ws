struct Solution;
impl Solution {

    fn backtrack(graph: &Vec<Vec<i32>>, results: &mut Vec<Vec<i32>>, mut path: Vec<i32>, curr_node: usize) {
        if curr_node == graph.len() - 1 {
            results.push(path);
            return;
        }

        for &neighbour in graph[curr_node].iter() {
            path.push(neighbour);
            Self::backtrack(graph, results, path.to_vec(), neighbour as usize);
            path.pop();
        }

    }

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results = vec![];
        Self::backtrack(&graph, &mut results, vec![0], 0);
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let graph = vec![vec![1,2],vec![3],vec![3],vec![]];
        let result = Solution::all_paths_source_target(graph);
        let out = vec![vec![0,1,3],vec![0,2,3]];
        assert_eq!(result, out);
    }

    #[test]
    fn t2() {
        let graph = vec![vec![4,3,1],vec![3,2,4],vec![3],vec![4],vec![]];
        let result = Solution::all_paths_source_target(graph);
        let out = vec![vec![0,4],vec![0,3,4],vec![0,1,3,4],vec![0,1,2,3,4],vec![0,1,4]];
        assert_eq!(result, out);
    }

}
