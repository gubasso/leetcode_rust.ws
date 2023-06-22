use std::collections::VecDeque;

struct RecentCounter {
    deque: VecDeque<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter { deque: VecDeque::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.deque.push_back(t);

        while self.deque.front().is_some() && *self.deque.front().unwrap() < t - 3000 {
            self.deque.pop_front();
        }

        self.deque.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let calls = vec![vec![],vec![1],vec![100],vec![3001],vec![3002]];
        let mut obj = RecentCounter::new();
        let mut ret: i32 = 0;
        for v in calls {
            if let Some(&call) = v.first() {
                ret = obj.ping(call);
            }
        }
        assert_eq!(ret, 3);
    }

}
