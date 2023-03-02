pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {

    0
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

    // #[test]
    // fn case_2() {
    //     let board = vec![vec![-1,-1], vec![-1,3]];
    //     let result = snakes_and_ladders(board);
    //     assert_eq!(result, 1);
    // }

}
