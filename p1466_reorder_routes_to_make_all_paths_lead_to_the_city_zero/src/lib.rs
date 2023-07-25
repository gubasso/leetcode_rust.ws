use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut roads = HashSet::new();
        let mut seen = HashSet::new();

        for cities in connections.iter() {
            graph.entry(cities[0]).or_insert(vec![]).push(cities[1]);
            graph.entry(cities[1]).or_insert(vec![]).push(cities[0]);
            roads.insert((cities[0],cities[1]));
        }
        println!("\n");

        let bfs = |node: i32, seen: &mut HashSet<i32>| -> i32 {
            let mut reorder_count = 0;
            let mut queue = VecDeque::from([node]);
            seen.insert(node);

            while let Some(nd) = queue.pop_front() {
                for dest in graph.get(&nd).into_iter().flatten() {
                    if seen.contains(dest) {
                        continue;
                    }
                    seen.insert(*dest);
                    if roads.contains(&(nd, *dest)) {
                        reorder_count += 1;
                    }
                    queue.push_back(*dest);
                }

            }

            reorder_count
        };


        bfs(0, &mut seen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let input = vec![vec![0,1],vec![1,3],vec![2,3],vec![4,0],vec![4,5]];
        let result = Solution::min_reorder(6, input);
        assert_eq!(result, 3);
    }

    #[test]
    fn t2() {
        let input = vec![vec![1,0],vec![1,2],vec![3,2],vec![3,4]];
        let result = Solution::min_reorder(5, input);
        assert_eq!(result, 2);
    }

    #[test]
    fn t3() {
        let input = vec![vec![1,0],vec![2,0]];
        let result = Solution::min_reorder(3, input);
        assert_eq!(result, 0);
    }

}
