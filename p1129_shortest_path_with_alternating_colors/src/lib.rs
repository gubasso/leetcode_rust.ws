

struct Solution;

use std::collections::{VecDeque, HashSet};
impl Solution {
    pub fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        const RED: usize = 0;
        const BLUE: usize = 1;
        let nu = n as usize;
        let mut ans = vec![i32::MAX; nu];
        let mut graph  = vec![vec![vec![]; nu]; 2];
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        for edges_pack in [(RED, red_edges), (BLUE, blue_edges)] {
            let color = edges_pack.0;
            let edges = edges_pack.1;
            for e in edges {
                let from = e[0] as usize;
                let to = e[1] as usize;
                graph[color][from].push(to);
            }
        }

        for color in [RED, BLUE] {
            if let Some(_) = graph.get(color).and_then(|e| e.get(0)) {
                // node, color, steps
                queue.push_back((0,color,0));
                // node, color
                seen.insert((0,color));
            }
        }

        while let Some((node, color, steps)) = queue.pop_front() {
            ans[node] = i32::min(ans[node], steps);
            for &neighbour in graph[color][node].iter() {
                if seen.insert((neighbour, 1-color)) {
                    queue.push_back((neighbour, 1-color, steps+1));
                }
            }
        }

        ans.into_iter().map(|val| {
            if val == i32::MAX {
                -1
            } else {
                val
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let red_edges = vec![vec![0,1],vec![1,2]];
        let blue_edges = vec![];
        let result = Solution::shortest_alternating_paths(3, red_edges, blue_edges);
        assert_eq!(result, vec![0,1,-1]);
    }

    #[test]
    fn t2() {
        let red_edges = vec![vec![0,1]];
        let blue_edges = vec![vec![2,1]];
        let result = Solution::shortest_alternating_paths(3, red_edges, blue_edges);
        assert_eq!(result, vec![0,1,-1]);
    }

}
