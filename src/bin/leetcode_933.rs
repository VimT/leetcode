//! 最近的请求次数

use std::collections::VecDeque;

struct RecentCounter {
    v: VecDeque<i32>,
}


impl RecentCounter {
    fn new() -> Self {
        Self { v: VecDeque::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while let Some(v) = self.v.front() {
            if *v < t - 3000 {
                self.v.pop_front().unwrap();
            } else { break; }
        }
        self.v.push_back(t);
        self.v.len() as i32
    }
}

fn main() {
    let mut rc = RecentCounter::new();
    assert_eq!(rc.ping(1), 1);     // requests = [1]，范围是 [-2999,1]，返回 1
    assert_eq!(rc.ping(100), 2);   // requests = [1, 100]，范围是 [-2900,100]，返回 2
    assert_eq!(rc.ping(3001), 3);  // requests = [1, 100, 3001]，范围是 [1,3001]，返回 3
    assert_eq!(rc.ping(3002), 3);  // requests = [1, 100, 3001, 3002]，范围是 [2,3002]，返回 3
}