use std::collections::HashMap;
use std::cmp;
use std::cmp::Ordering;

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

#[derive(Debug, Clone)]
struct DijItem {
    node: i32,
    shortest_distance: f64,
    node_from: Option<i32>,
}

#[derive(Debug)]
struct DijkstrasTable(HashMap<i32, DijItem>);

impl DijkstrasTable {
    pub fn new(board: &Board) -> Self {
        let mut table: HashMap<i32, DijItem> = HashMap::new();
        for board_item in board.0.iter() {
            let shortest_distance = if board_item.node == 1 { 0.0 } else { f64::INFINITY };
            let dij_item = DijItem {
                node: board_item.node,
                shortest_distance,
                node_from: None
            };
            table.insert(dij_item.node, dij_item.clone());
        }
        Self(table)
    }
}

fn cmp_f64_dij_item(a: &DijItem, b: &DijItem) -> Ordering {
    if a.shortest_distance < b.shortest_distance {
        return Ordering::Less;
    } else if a.shortest_distance > b.shortest_distance {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

#[derive(Debug)]
struct UnvisitedNodes(HashMap<i32, DijItem>);

impl UnvisitedNodes {
    pub fn new(dij_tab: &DijkstrasTable) -> Self {
        Self(dij_tab.0.clone())
    }
}

fn get_next_node(un_vis: &mut UnvisitedNodes) -> Option<DijItem> {
    let mut un_vis_vec: Vec<DijItem> = Vec::new();
    for (_k, v) in un_vis.0.iter() {
        un_vis_vec.push(v.clone());
    }
    un_vis_vec.sort_by(cmp_f64_dij_item);
    let next_node = un_vis_vec.remove(0);
    un_vis.0.remove(&next_node.node).clone()
}

pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    let new_board: Board = Board::new(board);
    // println!("new_board {:?}", new_board);
    let mut dijkstras_table: DijkstrasTable = DijkstrasTable::new(&new_board);
    // println!("dijkstras_table {:?}", dijkstras_table);
    let mut unvisited_nodes: UnvisitedNodes = UnvisitedNodes::new(&dijkstras_table);
    let mut count_interations: i32 = 0;
    let mut next_node_copy = get_next_node(&mut unvisited_nodes);
    // println!("unvisited_nodes {:?}", unvisited_nodes);
    while let Some(ref node_dij) = next_node_copy {
        println!("\n### THIS NODE {:?}\n", node_dij);
        let min_neighbour: i32 = cmp::min(node_dij.node+1, new_board.0.len() as i32);
        let max_neighbour: i32 = cmp::min(node_dij.node+6, new_board.0.len() as i32);
        let neighbours: Vec<i32> = if min_neighbour == node_dij.node { vec![] } else { (min_neighbour..=max_neighbour).collect() };
        for neighbour in neighbours.iter() {
            let row_board_tb = *neighbour as usize - 1;
            let neighbour_ok = if let Some(node) = &new_board.0[row_board_tb].jump_to_node {
                node
            } else {
                neighbour
            };
            let default_cost: f64 = 1.0;
            let board_item_neighbour_references_to = &new_board.0[*neighbour_ok as usize - 1];
            println!("board item neighbour_ok refs to: {:?}", board_item_neighbour_references_to);

            if let Some(dij_row_neighbour) = dijkstras_table.0.get_mut(neighbour_ok) {
                dij_row_neighbour.shortest_distance = if default_cost < dij_row_neighbour.shortest_distance { default_cost } else { dij_row_neighbour.shortest_distance };
                dij_row_neighbour.node_from = Some(node_dij.node);
                dij_row_neighbour.shortest_distance = if dij_row_neighbour.shortest_distance > 1.0 { 1.0 } else {dij_row_neighbour.shortest_distance};
                if let Some(_val) = unvisited_nodes.0.get(neighbour_ok) {
                    unvisited_nodes.0.insert(*neighbour_ok, dij_row_neighbour.clone());
                }
            }
            println!("dij_row_neighbour: {:?}", dijkstras_table.0.get(neighbour_ok));
        }
        next_node_copy = get_next_node(&mut unvisited_nodes);
        count_interations += 1;
        if count_interations == 1 {break;}
    }

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
