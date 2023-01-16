//! 找到数据流中的连续整数

use std::collections::{HashMap, VecDeque};

struct DataStream {
    q: VecDeque<i32>,
    k: usize,
    value: i32,
    map: HashMap<i32, usize>,
}


impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        Self {
            q: VecDeque::with_capacity(k as usize + 1),
            k: k as usize,
            value,
            map: HashMap::new(),
        }
    }

    fn consec(&mut self, num: i32) -> bool {
        self.q.push_back(num);
        if self.q.len() > self.k {
            *self.map.entry(self.q.pop_front().unwrap()).or_default() -= 1;
        }
        *self.map.entry(num).or_default() += 1;
        self.map.get(&self.value).map(|x| *x == self.k).unwrap_or(false)
    }
}


fn main() {
    let mut ds = DataStream::new(4, 3); // value = 4, k = 3
    assert_eq!(ds.consec(4), false); // 数据流中只有 1 个整数，所以返回 False 。
    assert_eq!(ds.consec(4), false); // 数据流中只有 2 个整数 由于 2 小于 k ，返回 False 。
    assert_eq!(ds.consec(4), true); // 数据流最后 3 个整数都等于 value， 所以返回 True 。
    assert_eq!(ds.consec(3), false); // 最后 k 个整数分别是 [4,4,3] 。 由于 3 不等于 value ，返回 False 。
}
