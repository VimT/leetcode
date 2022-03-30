//! 基于时间的键值存储

use std::collections::{BTreeMap, HashMap};

struct TimeMap {
    tree: HashMap<String, BTreeMap<i32, String>>,
}


impl TimeMap {
    fn new() -> Self {
        Self { tree: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.tree.entry(key).or_default().insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(v) = self.tree.get(&key) {
            if let Some(v) = v.range(..=timestamp).last() {
                return v.1.clone();
            }
        }
        String::new()
    }
}

fn main() {
    let mut tm = TimeMap::new();
    tm.set(String::from("foo"), String::from("bar"), 1);  // 存储键 "foo" 和值 "bar" ，时间戳 timestamp = 1
    tm.get(String::from("foo"), 1);         // 返回 "bar"
    tm.get(String::from("foo"), 3);         // 返回 "bar", 因为在时间戳 3 和时间戳 2 处没有对应 "foo" 的值，所以唯一的值位于时间戳 1 处（即 "bar"） 。
    tm.set(String::from("foo"), String::from("bar2"), 4); // 存储键 "foo" 和值 "bar2" ，时间戳 timestamp = 4
    tm.get(String::from("foo"), 4);         // 返回 "bar2"
    tm.get(String::from("foo"), 5);         // 返回 "bar2"
}
