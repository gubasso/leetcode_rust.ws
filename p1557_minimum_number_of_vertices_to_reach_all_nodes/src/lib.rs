struct Solution;
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        edges.into_iter().fold((0..n).collect::<Vec<i32>>(), |mut vec, edge| {
            vec[edge[1] as usize] = -1;
            vec
        }).into_iter().filter(|&e| e != -1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let input = vec![vec![0,1],vec![0,2],vec![2,5],vec![3,4],vec![4,2]];
        let result = Solution::find_smallest_set_of_vertices(6, input);
        assert_eq!(result, vec![0,3]);
    }

    #[test]
    fn t2() {
        let input = vec![vec![0,1],vec![2,1],vec![3,1],vec![1,4],vec![2,4]];
        let result = Solution::find_smallest_set_of_vertices(5, input);
        assert_eq!(result, vec![0,2,3]);
    }

}
