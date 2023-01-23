//! 序列顺序查询

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct SORTracker {
    /* 小根堆存放i-1个最大的值, 大根堆堆顶存放第i个最大值 */
    min_heap: BinaryHeap<(i32, String)>,
    max_heap: BinaryHeap<(i32, Reverse<String>)>,
}

impl SORTracker {
    fn new() -> Self {
        Self {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }

    fn add(&mut self, name: String, score: i32) {
        self.min_heap.push((-score, name));
        let (s, n) = self.min_heap.pop().unwrap();
        self.max_heap.push((-s, Reverse(n)));
    }

    fn get(&mut self) -> String {
        let (s, Reverse(n)) = self.max_heap.pop().unwrap();
        let result = n.clone();
        self.min_heap.push((-s, n));
        result
    }
}


fn main() {
    let mut tracker = SORTracker::new(); // 初始化系统
    tracker.add("bradford".to_string(), 2); // 添加 name="bradford" 且 score=2 的景点。
    tracker.add("branford".to_string(), 3); // 添加 name="branford" 且 score=3 的景点。
    assert_eq!(tracker.get(), "branford");              // 从好带坏的景点为：branford ，bradford 。
    // 注意到 branford 比 bradford 好，因为它的 评分更高 (3 > 2) 。
    // 这是第 1 次调用 get() ，所以返回最好的景点："branford" 。
    tracker.add("alps".to_string(), 2);     // 添加 name="alps" 且 score=2 的景点。
    assert_eq!(tracker.get(), "alps");              // 从好到坏的景点为：branford, alps, bradford 。
    // 注意 alps 比 bradford 好，虽然它们评分相同，都为 2 。
    // 这是因为 "alps" 字典序 比 "bradford" 小。
    // 返回第 2 好的地点 "alps" ，因为当前为第 2 次调用 get() 。
    tracker.add("orland".to_string(), 2);   // 添加 name="orland" 且 score=2 的景点。
    assert_eq!(tracker.get(), "bradford");              // 从好到坏的景点为：branford, alps, bradford, orland 。
    // 返回 "bradford" ，因为当前为第 3 次调用 get() 。
    tracker.add("orlando".to_string(), 3);  // 添加 name="orlando" 且 score=3 的景点。
    assert_eq!(tracker.get(), "bradford");              // 从好到坏的景点为：branford, orlando, alps, bradford, orland 。
    // 返回 "bradford".
    tracker.add("alpine".to_string(), 2);   // 添加 name="alpine" 且 score=2 的景点。
    assert_eq!(tracker.get(), "bradford");              // 从好到坏的景点为：branford, orlando, alpine, alps, bradford, orland 。
    // 返回 "bradford" 。
    assert_eq!(tracker.get(), "orland");              // 从好到坏的景点为：branford, orlando, alpine, alps, bradford, orland 。
    // 返回 "orland" 。
}
