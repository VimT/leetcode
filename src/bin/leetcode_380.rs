//! O(1) 时间插入、删除和获取随机元素

use std::collections::HashMap;

use rand::prelude::random;

#[derive(Default)]
struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
}


impl RandomizedSet {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) { return false; }
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
        return self.list[random::<usize>() % self.list.len()];
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
