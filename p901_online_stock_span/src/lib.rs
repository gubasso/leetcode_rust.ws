struct StockSpanner {
    dec_stack: Vec<(i32, i32)>,
}

impl StockSpanner {

    fn new() -> Self {
        StockSpanner { dec_stack: Vec::new() }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        while self.dec_stack.last().map_or(false, |&last| price >= last.0) {
            span += self.dec_stack.pop().unwrap().1;
        }
        self.dec_stack.push((price, span));
        span
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let mut obj = StockSpanner::new();
        let input = vec![vec![100], vec![80], vec![60], vec![70], vec![60], vec![75], vec![85]];
        let output = vec![1, 1, 1, 2, 1, 4, 6];
        for i in 0..input.len() {
            let result = obj.next(input[i][0]);
            assert_eq!(result, output[i]);
        }
    }

}
