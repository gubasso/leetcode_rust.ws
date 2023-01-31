use std::cmp;

pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    let n: i32  = board.len() as i32;
    let last = i32::pow(n, 2);
    let mut unvisited_nodes: Vec<i32> = (1..=last).collect();
    let mut visited_nodes: Vec<i32> = Vec::new();
    let mut dijkstras_table: Vec<Vec<i32>> = vec![Vec::new(), Vec::new(), Vec::new()];
    let mut adjancency_matrix: Vec<Vec<i32>> = Vec::new();

    println!("n: {}", n);
    let get_is_row_even = |&row_sequencial: &i32| -> bool {
        0 == row_sequencial % 2
    };

    let get_row_seq = |&node: &i32, &n: &i32| -> i32 {
        (node - 1) / n
    };

    let get_coord = |&node: &i32| -> (i32, i32) {
        let row_sequencial: i32 = get_row_seq(&node, &n);
        let row_arr: i32 = n - 1 - row_sequencial;
        let col_arr: i32 = if get_is_row_even(&row_sequencial) {
            (node - (n * row_sequencial)) - 1
        } else {
            ((row_sequencial + 1)*n) - node
        };
        (row_arr, col_arr)
    };

    // TODO: solve relationship when snake/ladder
    // 2 will be equal 15... for everyone refering to 2
    let mut node: i32 = 1;
    for b_line in board.into_iter().rev() {
        println!("board line: {:?}", b_line);
        let row_sequencial: i32 = get_row_seq(&node, &n);
        let is_row_even: bool = get_is_row_even(&row_sequencial);
        println!("row seq: {row_sequencial}, is row even: {is_row_even}");
        let line_right_order: Vec<i32> = if is_row_even {
            b_line.to_vec()
        } else {
            b_line.into_iter().rev().collect()
        };
        for snake_ladder_val in line_right_order.iter() {
            println!("node: {node}. snake_ladder_val: {:?}", snake_ladder_val);
            let (row_arr, col_arr) = get_coord(&node);
            println!("array: [r][c]: {row_arr},{col_arr}");
            let mut relationship_array: Vec<i32> = vec![0; last as usize];
            let node_related_to: Vec<i32> = ((node+1)..=(cmp::min(node+6, last))).collect();
            println!("node_related_to: {:?}", node_related_to);
            // for node_related in node_related_to.iter() {
            //     relationship_array[(node_related - 1) as usize] = 1;
            // }
            println!("relationship_array: {:?}", relationship_array);
            adjancency_matrix.push(relationship_array);
            node += 1;
        }
    }

    // for node in unvisited_nodes.iter() {
    //     let (row, column) = get_coord(node);
    //     println!("node: {}, coord: {},{}", node, row, column);
    //     println!("from board n: {}", board[row as usize][column as usize]);
    // }
    //
    // println!("{:?}", adjancency_matrix);
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
