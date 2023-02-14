//! 敲击计数器

use std::cmp::Ordering;

struct HitCounter {
    hits: Vec<i32>,
}


impl HitCounter {
    fn new() -> Self {
        Self { hits: vec![] }
    }

    fn hit(&mut self, timestamp: i32) {
        self.hits.push(timestamp);
    }

    fn get_hits(&self, timestamp: i32) -> i32 {
        let right = self.hits.binary_search_by(|mid| mid.cmp(&timestamp).then(Ordering::Less)).unwrap_err();
        let left = self.hits.binary_search_by(|mid| mid.cmp(&(timestamp - 299)).then(Ordering::Greater)).unwrap_err();
        (right - left) as i32
    }
}


fn main() {
    let mut counter = HitCounter::new();
    counter.hit(1);// 在时刻 1 敲击一次。
    counter.hit(3);// 在时刻 2 敲击一次。
    counter.hit(3);// 在时刻 3 敲击一次。
    assert_eq!(counter.get_hits(4), 3);// 在时刻 4 统计过去 5 分钟内的敲击次数, 函数返回 3 。
    assert_eq!(counter.get_hits(3), 3);// 在时刻 4 统计过去 5 分钟内的敲击次数, 函数返回 3 。
    counter.hit(300);// 在时刻 300 敲击一次。
    assert_eq!(counter.get_hits(300), 4); // 在时刻 300 统计过去 5 分钟内的敲击次数，函数返回 4 。
    assert_eq!(counter.get_hits(301), 3); // 在时刻 301 统计过去 5 分钟内的敲击次数，函数返回 3 。
}
