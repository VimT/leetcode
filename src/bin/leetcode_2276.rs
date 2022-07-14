//! 统计区间中的整数数目

use std::collections::BTreeSet;

struct CountIntervals {
    set: BTreeSet<(i32, i32)>,
    count: i32,
}


impl CountIntervals {
    fn new() -> Self {
        Self { set: BTreeSet::new(), count: 0 }
    }

    // 区间并集模板
    fn add(&mut self, mut left: i32, mut right: i32) {
        // 找到第一个右边界right大于等于left的区间
        let mut it = self.set.range((left - 1, i32::MIN)..);
        let mut remove = vec![];
        while let Some(&range) = it.next() {
            if range.1 > right + 1 { break; }
            left = left.min(range.1);
            right = right.max(range.0);
            self.count -= range.0 - range.1 + 1;
            remove.push(range);
        }
        self.count += right - left + 1;
        for range in remove {
            self.set.remove(&range);
        }
        // 集合按区间右边界right从小到大维护区间
        self.set.insert((right, left));
    }

    fn count(&self) -> i32 {
        self.count
    }
}


fn main() {
    let mut ci = CountIntervals::new();
    ci.add(2, 3);
    ci.add(7, 10);
    assert_eq!(ci.count(), 6);
    ci.add(5, 8);
    assert_eq!(ci.count(), 8);
}
