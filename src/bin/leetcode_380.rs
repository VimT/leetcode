//! O(1) 时间插入、删除和获取随机元素
use std::collections::HashMap;

use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(Default)]
struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
    rng: Option<ThreadRng>,
}


impl RandomizedSet {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, val: i32) -> bool {
        let has = self.map.contains_key(&val);
        if has { return false; }
        self.list.push(val);
        self.map.insert(val, self.list.len() - 1);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        return match self.map.remove(&val) {
            Some(index) => {
                let _len = self.list.len();
                self.list.swap_remove(index);
                if index < self.list.len() {
                    self.map.insert(self.list[index], index);
                }
                true
            }
            None => { false }
        };
    }

    fn get_random(&mut self) -> i32 {
        let rng = self.rng.get_or_insert(rand::thread_rng());
        return self.list[rng.gen_range(0, self.list.len())];
    }
}


fn main() {
    let mut obj = RandomizedSet::new();
    let _ret_1: bool = obj.remove(0);
    let _ret_1: bool = obj.remove(0);
    let _ret_2: bool = obj.insert(0);
    let _ret = obj.get_random();
    let _ret_2: bool = obj.remove(0);
    let _ret_3 = obj.insert(0);
}
