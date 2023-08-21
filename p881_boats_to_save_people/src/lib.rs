struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();
        let mut boats = 0;
        let (mut i, mut j) = (0, (people.len()-1) as i32);

        while i <= j {
            if people[i as usize] + people[j as usize] <= limit {
                i += 1;
            }

            j -= 1;
            boats += 1;
        }

        boats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let people = vec![1,2];
        let limit = 3;
        let result = Solution::num_rescue_boats(people,limit);
        assert_eq!(result, 1);
    }

    #[test]
    fn t2() {
        let people = vec![3,2,2,1];
        let limit = 3;
        let result = Solution::num_rescue_boats(people,limit);
        assert_eq!(result, 3);
    }

    #[test]
    fn t3() {
        let people = vec![3,5,3,4];
        let limit = 5;
        let result = Solution::num_rescue_boats(people,limit);
        assert_eq!(result, 4);
    }

    #[test]
    fn t4() {
        let people = vec![5,1,4,2];
        let limit = 6;
        let result = Solution::num_rescue_boats(people,limit);
        assert_eq!(result, 2);
    }

    #[test]
    fn t5() {
        let people = vec![2,2];
        let limit = 6;
        let result = Solution::num_rescue_boats(people,limit);
        assert_eq!(result, 1);
    }

}
