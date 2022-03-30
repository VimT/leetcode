//! O(1) 时间插入、删除和获取随机元素 - 允许重复

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

struct RandomizedCollection {
    idx: HashMap<i32, HashSet<usize>>,
    nums: Vec<i32>,
    rng: ThreadRng,
}

impl RandomizedCollection {
    fn new() -> Self {
        Self { idx: HashMap::new(), nums: vec![], rng: thread_rng() }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.nums.push(val);
        match self.idx.entry(val) {
            Entry::Occupied(mut v) => {
                v.get_mut().insert(self.nums.len() - 1);
                false
            }
            Entry::Vacant(v) => {
                v.insert(HashSet::new()).insert(self.nums.len() - 1);
                true
            }
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Entry::Occupied(mut value) = self.idx.entry(val) {
            let set = value.get_mut();
            let idx = *set.iter().next().unwrap();
            self.nums[idx] = *self.nums.last().unwrap();

            if set.len() == 1 {
                value.remove();
            } else {
                set.remove(&idx);
            }
            if let Some(changed) = self.idx.get_mut(&self.nums[idx]) {
                changed.remove(&(self.nums.len() - 1));
                if idx < self.nums.len() - 1 {
                    changed.insert(idx);
                }
            }
            self.nums.pop().unwrap();
            return true;
        }
        false
    }

    fn get_random(&mut self) -> i32 {
        let idx = self.rng.gen_range(0, self.nums.len());
        self.nums[idx]
    }
}

fn main() {
    // 初始化一个空的集合。
    let mut collection = RandomizedCollection::new();

    // 向集合中插入 1 。返回 true 表示集合不包含 1 。
    assert_eq!(collection.insert(1), true);

    // 向集合中插入另一个 1 。返回 false 表示集合包含 1 。集合现在包含 [1,1] 。
    assert_eq!(collection.insert(1), false);

    // 向集合中插入 2 ，返回 true 。集合现在包含 [1,1,2] 。
    assert_eq!(collection.insert(2), true);

    // getRandom 应当有 2/3 的概率返回 1 ，1/3 的概率返回 2 。
    println!("{}", collection.get_random());

    // 从集合中删除 1 ，返回 true 。集合现在包含 [1,2] 。
    assert_eq!(collection.remove(1), true);

    // getRandom 应有相同概率返回 1 和 2 。
    println!("{}", collection.get_random());
}
