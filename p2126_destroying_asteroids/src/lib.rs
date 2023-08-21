struct Solution;
impl Solution {
    pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        asteroids.sort();
        let mut mass = mass as u64;
        for a in asteroids {
            let a = a as u64;
            if a > mass {
                return false;
            }
            mass += a;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let mass = 10;
        let asteroids = vec![3,9,19,5,21];
        let result = Solution::asteroids_destroyed(mass, asteroids);
        assert_eq!(result, true);
    }

    #[test]
    fn t2() {
        let mass = 5;
        let asteroids = vec![4,9,23,4];
        let result = Solution::asteroids_destroyed(mass, asteroids);
        assert_eq!(result, false);
    }

}
