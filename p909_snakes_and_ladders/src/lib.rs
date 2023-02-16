use std::cmp;

#[derive(Debug)]
struct Node(i32);

#[derive(Debug)]
struct DijItem {
    val: f64,
    node_from: Option<Node>,
}

#[derive(Debug)]
struct Dijkstras {
    table: Vec<DijItem>,
    unvisited_nodes: Vec<i32>,
}

impl Dijkstras {
    pub fn new(board_side_size: i32) -> Self {
        let last_node: i32 = i32::pow(board_side_size, 2);
        let unvisited_nodes: Vec<i32> = (1..=last_node).collect();
        let table: Vec<DijItem> = unvisited_nodes.iter().map(|i| {
            let val = if *i == 1 { 0.0 } else { f64::INFINITY };
            return DijItem {
                val,
                node_from: None
            }
        }).collect();
        Self {
            table,
            unvisited_nodes
        }
    }
}

impl Iterator for Dijkstras {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.unvisited_nodes.first();

    }
}

pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    // let n: i32  = board.len() as i32;
    //ok let last = i32::pow(n, 2);
    //ok let mut unvisited_nodes: Vec<i32> = (1..=last).collect();
    //ok let mut visited_nodes: Vec<i32> = Vec::new();
    //ok let mut dijkstras_table: Vec<(i32, f64, Option<i32>)> = vec![(0,f64::INFINITY, None); last as usize];
    //ok println!("created dijkstras {:?}", dijkstras_table);
    //
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

    let dj_tb = DijkstrasTable::new(board.len() as i32);
    println!("{:?}", dj_tb);


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
