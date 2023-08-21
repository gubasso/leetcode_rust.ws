struct Solution;
use std::cmp::Reverse;
impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_by_key(|vec| Reverse(vec[1]));
        let mut total_units = 0;
        let mut i = 0;

        while truck_size > 0 && i < box_types.len() {
            let (mut n_boxes, units) = (box_types[i][0], box_types[i][1]);
            while truck_size > 0 && n_boxes > 0 {
                total_units += units;
                n_boxes -= 1;
                truck_size -= 1;
            }
            i += 1;
        }

        total_units
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let box_types = vec![vec![1,3],vec![2,2],vec![3,1]];
        let truck_size = 4;
        let result = Solution::maximum_units(box_types, truck_size);
        assert_eq!(result, 8);
    }

    #[test]
    fn t2() {
        let box_types = vec![vec![5,10],vec![2,5],vec![4,7],vec![3,9]];
        let truck_size = 10;
        let result = Solution::maximum_units(box_types, truck_size);
        assert_eq!(result, 91);
    }

}
