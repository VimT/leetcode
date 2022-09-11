//! 设计文件系统

use std::collections::HashMap;

struct FileSystem {
    store: HashMap<String, i32>,
}

impl FileSystem {
    fn new() -> Self {
        Self { store: HashMap::new() }
    }

    fn create_path(&mut self, path: String, value: i32) -> bool {
        let index = path.rfind('/').unwrap();
        let parent = &path[0..index];
        if self.store.contains_key(&path) || (!parent.is_empty() && !self.store.contains_key(parent)) {
            return false;
        }
        self.store.insert(path, value);
        true
    }

    fn get(&self, path: String) -> i32 {
        self.store.get(&path).cloned().unwrap_or(-1)
    }
}


fn main() {
    let mut fs = FileSystem::new();
    assert_eq!(fs.create_path(String::from("/leet"), 1), true); // 返回 true
    assert_eq!(fs.create_path(String::from("/leet/code"), 2), true); // 返回 true
    assert_eq!(fs.get(String::from("/leet/code")), 2); // 返回 2
    assert_eq!(fs.create_path(String::from("/c/d"), 1), false); // 返回 false 因为父路径 "/c" 不存在。
    assert_eq!(fs.get(String::from("/c")), -1); // 返回 -1 因为该路径不存在。
}
