use std::collections::HashMap;

pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    let n: i32  = board.len() as i32;
    let last = i32::pow(n, 2);
    let mut unvisited_nodes: Vec<i32> = (1..=last).collect();
    let mut visited_nodes: Vec<i32> = Vec::new();
    let mut dijkstras_table: Vec<Vec<i32>> = vec![Vec::new(), Vec::new(), Vec::new()];
    let mut adjancency_matrix: Vec<Vec<i32>> = Vec::new();

    println!("n: {}", n);

    for node in unvisited_nodes.iter() {
        let row: i32 = (node - 1) / n;
        let column: i32 = if let 0 = row % 2 {
            (node - (n * row)) - 1
        } else {
            ((row + 1)*n) - node
        };
        let node_related_to: Vec<i32> = ((node+1)..=(node+6)).collect();
        println!("node: {}", node);
        println!("node_related_to: {:?}", node_related_to);
        println!("row: {}", row);
        println!("column: {}", column);
        let mut relationship_array: Vec<i32> = Vec::new();
        adjancency_matrix.push(relationship_array)
    }

    // println!("{:?}", unvisited_nodes);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let board = vec![vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,35,-1,-1,13,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,15,-1,-1,-1,-1]];
        let result = snakes_and_ladders(board);
        assert_eq!(result, -1);
    }
}
