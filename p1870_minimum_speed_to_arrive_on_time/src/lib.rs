struct Solution;
impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        if dist.len() > hour.ceil() as usize {
            return -1;
        }

        let check = |s: f64| -> bool {
            let mut t: f64 = 0.0;
            for &d in dist.iter() {
                t = t.ceil();
                t += d as f64 / s;
                if t > hour {
                    return false;
                }
            }
            true
        };

        let (mut l, mut r) = (0, 10_i32.pow(7)+1);

        while l < r {
            let speed = l + (r - l)/2;
            match check(speed as f64) {
                false => l = speed + 1,
                true => r = speed
            }
        }

        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let dist = vec![1,3,2];
        let hour = 6.0;
        let result = Solution::min_speed_on_time(dist,hour);
        assert_eq!(result, 1);
    }

    #[test]
    fn t2() {
        let dist = vec![1,3,2];
        let hour = 2.7;
        let result = Solution::min_speed_on_time(dist,hour);
        assert_eq!(result, 3);
    }

    #[test]
    fn t3() {
        let dist = vec![1,3,2];
        let hour = 1.9;
        let result = Solution::min_speed_on_time(dist,hour);
        assert_eq!(result, -1);
    }

}
