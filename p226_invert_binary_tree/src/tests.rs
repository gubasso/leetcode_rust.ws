use super::*;

#[test]
fn t1() {
    println!("\n");
    let vec_input = vec![4,2,7,1,3,6,9];
    let vec_answer = vec![4,7,2,9,6,3,1];
    let tree_input = convert_vec_to_tree_node(&vec_input, 0);
    let inv_tree_input = Solution::invert_tree(tree_input);
    let inv_vec = convert_tree_node_to_vec(&inv_tree_input);
    assert_eq!(inv_vec, vec_answer);
}

#[test]
fn t2() {
    println!("\n");
    let vec_input = vec![2,1,3];
    let vec_answer = vec![2,3,1];
    let tree_input = convert_vec_to_tree_node(&vec_input, 0);
    let inv_tree_input = Solution::invert_tree(tree_input);
    let inv_vec = convert_tree_node_to_vec(&inv_tree_input);
    assert_eq!(inv_vec, vec_answer);
}

#[test]
fn t3() {
    let vec_input = vec![];
    let vec_answer = vec![];
    let tree_input = convert_vec_to_tree_node(&vec_input, 0);
    let inv_tree_input = Solution::invert_tree(tree_input);
    let inv_vec = convert_tree_node_to_vec(&inv_tree_input);
    assert_eq!(inv_vec, vec_answer);
}

