//! 股票价格跨度

struct StockSpanner {
    pw: Vec<(i32, i32)>,
}


/// 神奇的单调栈
impl StockSpanner {
    fn new() -> Self {
        Self { pw: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut w = 1;
        while !self.pw.is_empty() && self.pw.last().unwrap().0 <= price {
            let (_, weight) = self.pw.pop().unwrap();
            w += weight;
        }
        self.pw.push((price, w));
        w
    }
}


fn main() {
    let mut ss = StockSpanner::new();
    assert_eq!(ss.next(100), 1); // 被调用并返回 1，
    assert_eq!(ss.next(80), 1); // 被调用并返回 1，
    assert_eq!(ss.next(60), 1); // 被调用并返回 1，
    assert_eq!(ss.next(70), 2); // 被调用并返回 2，
    assert_eq!(ss.next(60), 1); // 被调用并返回 1，
    assert_eq!(ss.next(75), 4); // 被调用并返回 4，
    assert_eq!(ss.next(85), 6); // 被调用并返回 6。
}