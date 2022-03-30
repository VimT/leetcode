//! 设计哈希映射

struct MyHashMap {
    bucket: Vec<Vec<(i32, i32)>>,
}

impl MyHashMap {
    fn new() -> Self {
        Self { bucket: vec![vec![]; 8000] }
    }

    fn put(&mut self, key: i32, value: i32) {
        let idx = key % 8000;
        for (k, v) in &mut self.bucket[idx as usize] {
            if *k == key {
                *v = value;
                return;
            }
        }
        self.bucket[idx as usize].push((key, value));
    }

    fn get(&self, key: i32) -> i32 {
        let idx = key % 8000;
        let slot = &self.bucket[idx as usize];
        for (k, v) in slot {
            if *k == key {
                return *v;
            }
        }
        return -1;
    }

    fn remove(&mut self, key: i32) {
        let idx = key % 8000;
        let slot = &mut self.bucket[idx as usize];
        let len = slot.len();
        for i in 0..len {
            if slot[i].0 == key {
                slot.swap_remove(i);
                return;
            }
        }
    }
}

fn main() {
    let mut map = MyHashMap::new();
    map.put(1, 1); // map 现在为 [[1,1]]
    map.put(2, 2); // map 现在为 [[1,1], [2,2]]
    assert_eq!(map.get(1), 1);    // 返回 1 ，map 现在为 [[1,1], [2,2]]
    assert_eq!(map.get(3), -1);    // 返回 -1（未找到），map 现在为 [[1,1], [2,2]]
    map.put(2, 1); // map 现在为 [[1,1], [2,1]]（更新已有的值）
    assert_eq!(map.get(2), 1);    // 返回 1 ，map 现在为 [[1,1], [2,1]]
    map.remove(2); // 删除键为 2 的数据，map 现在为 [[1,1]]
    assert_eq!(map.get(2), -1);    // 返回 -1（未找到），map 现在为 [[1,1]]
}
