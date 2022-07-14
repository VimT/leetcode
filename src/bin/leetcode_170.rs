//! 两数之和 III - 数据结构设计

use std::collections::HashMap;

struct TwoSum {
    cnt: HashMap<i32, i32>,
}


impl TwoSum {
    fn new() -> Self {
        Self { cnt: HashMap::new() }
    }

    fn add(&mut self, number: i32) {
        *self.cnt.entry(number).or_default() += 1;
    }

    fn find(&self, value: i32) -> bool {
        for (k, v) in &self.cnt {
            if k * 2 == value {
                if *v > 1 {
                    return true;
                }
            } else if self.cnt.contains_key(&(value - *k)) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut ts = TwoSum::new();
    ts.add(1);
    ts.add(3);
    ts.add(5);
    assert_eq!(ts.find(4), true);
    assert_eq!(ts.find(2), false);
    assert_eq!(ts.find(7), false);
}
