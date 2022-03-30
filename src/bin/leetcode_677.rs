//! 键值映射


use std::collections::BTreeMap;

struct MapSum {
    m: BTreeMap<String, i32>,
}


impl MapSum {
    fn new() -> Self {
        Self { m: BTreeMap::new() }
    }

    fn insert(&mut self, key: String, val: i32) {
        self.m.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        let mut result = 0;
        for (s, v) in self.m.range(prefix.clone()..) {
            if s.starts_with(&prefix) {
                result += *v;
            } else {
                break;
            }
        }
        result
    }
}


fn main() {
    let mut ms = MapSum::new();
    ms.insert(String::from("apple"), 3);
    assert_eq!(ms.sum(String::from("ap")), 3);
    ms.insert(String::from("app"), 2);
    assert_eq!(ms.sum(String::from("ap")), 5);
}
