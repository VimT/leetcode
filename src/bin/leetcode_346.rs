//! 数据流中的移动平均值

use std::collections::VecDeque;

struct MovingAverage {
    q: VecDeque<i32>,
    sum: i32,
    size: usize,
}


impl MovingAverage {
    fn new(size: i32) -> Self {
        Self {
            q: VecDeque::with_capacity(size as usize + 1),
            sum: 0,
            size: size as usize,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.q.push_back(val);
        self.sum += val;
        if self.q.len() > self.size {
            self.sum -= self.q.pop_front().unwrap();
        }
        self.sum as f64 / self.q.len() as f64
    }
}

fn main() {
    let mut ma = MovingAverage::new(3);
    assert_eq!(ma.next(1), 1.); // 返回 1.0 = 1 / 1
    assert_eq!(ma.next(10), 5.5); // 返回 5.5 = (1 + 10) / 2
    assert_eq!(ma.next(3), 4.666666666666667); // 返回 4.66667 = (1 + 10 + 3) / 3
    assert_eq!(ma.next(5), 6.); // 返回 6.0 = (10 + 3 + 5) / 3
}
