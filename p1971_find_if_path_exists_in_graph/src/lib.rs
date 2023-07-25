struct Solution;
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let nu = n as usize;
        let mut graph = vec![vec![]; nu];
        let mut seen = vec![false; nu];

        for e in edges {
            graph[e[0] as usize].push(e[1] as usize);
            graph[e[1] as usize].push(e[0] as usize);
        }

        let dfs = |start: usize, seen: &mut Vec<bool>| -> bool {
            let mut stack = vec![start];

            while let Some(nd) = stack.pop() {
                if nd == destination as usize {
                    return true;
                }
                for &neighbour in graph[nd].iter() {
                    if !seen[neighbour] {
                        seen[neighbour] = true;
                        stack.push(neighbour);
                    }
                }
            }

            false
        };

        seen[source as usize] = true;
        dfs(source as usize, &mut seen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let edges = vec![vec![0,1],vec![1,2],vec![2,0]];
        let result = Solution::valid_path(3, edges, 0, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let edges = vec![vec![0,1],vec![0,2],vec![3,5],vec![5,4],vec![4,3]];
        let result = Solution::valid_path(6, edges, 0, 5);
        assert_eq!(result, false);
    }

    #[test]
    fn t3() {
        let edges = vec![];
        let result = Solution::valid_path(1, edges, 0, 0);
        assert_eq!(result, true);
    }

}
