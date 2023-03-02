use std::collections::{HashMap, VecDeque};

fn get_position(board: &Vec<Vec<i32>>, node: i32) -> i32 {
    let lenght = board.len() as i32;
    let mut row = (node-1) / lenght;
    let mut column = (node-1) % lenght;
    if row % 2 != 0 {
        column = -(column - lenght+1);
    }
    row = -(row - lenght+1);
    board[row as usize][column as usize]

}

fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    let lenght = board.len() as i32;
    let total_nodes = lenght*lenght;
    let mut visited_nodes: HashMap<i32, i32> = HashMap::from([(1,0)]);
    let mut queue: VecDeque<i32> = VecDeque::from([1]);
    while let Some(node_from_q) = queue.pop_front() {
        for mut neighbour in node_from_q+1..=node_from_q+6 {
            let neighbour_position = get_position(&board, neighbour);
            if neighbour > total_nodes { return -1 };
            if neighbour_position != -1 { neighbour = neighbour_position };
            if neighbour == total_nodes {
                if let Some(node) = visited_nodes.get(&node_from_q) {
                    return node+1;
                }
            };
            if !visited_nodes.contains_key(&neighbour) {
                let path_val: i32 = if let Some(node) = visited_nodes.get(&node_from_q) {
                    node + 1
                } else { 0 };
                visited_nodes.insert(neighbour, path_val);
                queue.push_back(neighbour);
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let board = vec![vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,35,-1,-1,13,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,15,-1,-1,-1,-1]];
        let result = snakes_and_ladders(board);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_2() {
        let board = vec![vec![-1,-1], vec![-1,3]];
        let result = snakes_and_ladders(board);
        assert_eq!(result, 1);
    }

}
