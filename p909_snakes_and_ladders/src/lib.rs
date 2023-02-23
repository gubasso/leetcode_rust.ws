use std::cmp;

#[derive(Debug)]
struct BoardItem {
    node: i32,
    jump_to_node: Option<i32>,
}

#[derive(Debug)]
struct Board(Vec<BoardItem>);

impl Board {
    pub fn new(board: Vec<Vec<i32>>) -> Self {
        let mut new_board: Vec<BoardItem> = Vec::new();
        let mut node: i32 = 0;
        for (idx, row) in board.into_iter().rev().enumerate() {
            let is_row_even: bool = 0 == idx % 2;
            let row_in_order = if is_row_even { row } else { row.into_iter().rev().collect() };
            for jump_to_node in row_in_order.iter() {
                node += 1;
                new_board.push(BoardItem {
                    node,
                    jump_to_node: if *jump_to_node == -1 { None } else { Some(*jump_to_node) }
                })
            }
        }
        Self(new_board)
    }
}


#[derive(Debug)]
struct DijItem {
    node: i32,
    shortest_distance: f64,
    node_from: Option<i32>,
}

#[derive(Debug)]
struct DijkstrasTable(Vec<DijItem>);

impl DijkstrasTable {
    pub fn new(board: &Board) -> Self {
        let mut table: Vec<DijItem> = Vec::new();
        for board_item in board.0.iter() {
            let shortest_distance = if board_item.node == 1 { 0.0 } else { f64::INFINITY };
            table.push(DijItem {
                node: board_item.node,
                shortest_distance,
                node_from: None
            })
        }
        Self(table)
    }
}

pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    let new_board: Board = Board::new(board);
    // println!("new_board {:?}", new_board);
    let mut dijkstras_table: DijkstrasTable = DijkstrasTable::new(&new_board);
    println!("dijkstras_table {:?}", dijkstras_table);
    for board_item in new_board.0.iter() {
        // TODO max between two values... neighbours 36 max
        println!("board_item: {:?}", board_item);
        let min_neighbour: i32 = cmp::min(board_item.node+1, new_board.0.len() as i32);
        let max_neighbour: i32 = cmp::min(board_item.node+6, new_board.0.len() as i32);
        let neighbours: Vec<i32> = if min_neighbour == board_item.node { vec![] } else { (min_neighbour..=max_neighbour).collect() };
        println!("neighbours original: {:?}", neighbours);
        for neighbour in neighbours.iter() {
            // let row_board_tb = *neighbour as usize - 1;
            // let neighbour_ok = if let Some(node) = &new_board.0[row_board_tb].jump_to_node {
            //     node
            // } else {
            //     neighbour
            // };
            // let board_item_neighbour_references_to = &new_board.0[*neighbour_ok as usize - 1];
            // // println!("board item neighbour_ok refs to: {:?}", board_item_neighbour_references_to);
            // let row_dij_tab = *neighbour_ok as usize -1;
            // let mut dij_row = &mut dijkstras_table.0[row_dij_tab];
            // dij_row.shortest_distance = if dij_row.shortest_distance > 1.0 { 1.0 } else {dij_row.shortest_distance};
        }
    }
    // println!("{:?}", dijkstras_table);

    // let get_is_row_even = |&row_sequencial: &i32| -> bool {
    //     0 == row_sequencial % 2
    // };
    //
    // let get_row_seq = |&node: &i32, &n: &i32| -> i32 {
    //     (node - 1) / n
    // };
    //
    // let get_coord = |&node: &i32| -> (i32, i32) {
    //     let row_sequencial: i32 = get_row_seq(&node, &n);
    //     let row_arr: i32 = n - 1 - row_sequencial;
    //     let col_arr: i32 = if get_is_row_even(&row_sequencial) {
    //         (node - (n * row_sequencial)) - 1
    //     } else {
    //         ((row_sequencial + 1)*n) - node
    //     };
    //     (row_arr, col_arr)
    // };
    //
    //
    // for node in unvisited_nodes.iter() {
    //     let row_sequencial: i32 = get_row_seq(&node, &n);
    //     let (row_arr, col_arr) = get_coord(&node);
    //     let this_node_value_related = board[row_arr as usize][col_arr as usize];
    //     println!("### node: {node}, row_seq: {row_sequencial}, board[r][c]: {row_arr},{col_arr} ###");
    //     println!("### this node value related: {this_node_value_related}");
    //     if this_node_value_related != -1 { continue; };
    //     let node_is_related_to: Vec<i32> = ((node+1)..=(cmp::min(node+6, last))).collect();
    //     let node_row: usize = *node as usize -1;
    //     if *node == 1 {
    //         dijkstras_table[node_row] = (*node, 0.0, None);
    //     }
    //     for node_related in node_is_related_to.iter() {
    //         let (row_arr, col_arr) = get_coord(&node_related);
    //         let value_of_node_related = board[row_arr as usize][col_arr as usize];
    //         println!("node_related: {node_related} -> value_of_node_related: {value_of_node_related}");
    //         let neighbour = if value_of_node_related == -1 {
    //             node_related
    //         } else {
    //             &value_of_node_related
    //         };
    //         let dt_neigh_row = *neighbour as usize - 1;
    //         let neighbour_path_val = if dijkstras_table[dt_neigh_row].1 == f64::INFINITY { 0.0 } else { dijkstras_table[dt_neigh_row].1 };
    //         let sum_paths = dijkstras_table[node_row].1 + neighbour_path_val;
    //         println!("sum paths: {sum_paths}");
    //         println!("dijkstras_table[neigh_row]: {:?}", dijkstras_table[dt_neigh_row]);
    //         if *node == 1 {
    //             dijkstras_table[dt_neigh_row] = (*neighbour, 1.0, Some(*node));
    //             println!("{:?}", dijkstras_table[dt_neigh_row]);
    //         } else if dijkstras_table[dt_neigh_row].1 > sum_paths {
    //             println!("substitute for lower cost path");
    //             dijkstras_table[dt_neigh_row] = (*neighbour, sum_paths, Some(*node));
    //             println!("{:?}", dijkstras_table[dt_neigh_row]);
    //         }
    //
    //     }
    //     // println!("{:?}", dijkstras_table);
    //     if *node == 3 {
    //         break;
    //     }
    // }
    //
    // println!("{:?}", dijkstras_table);

    // let dj_tb = Dijkstras::new(board.len() as i32);
    // println!("{:?}", dj_tb);


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
