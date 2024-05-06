//! K 周期字符串需要的最少操作次数

use std::collections::HashMap;

pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
    let s = word.as_bytes();
    let k = k as usize;
    let len = s.len();
    let mut m = HashMap::new();
    for c in s.chunks(k) {
        *m.entry(c).or_insert(0) += 1;
    }
    (len / k - m.values().max().copied().unwrap()) as i32
}

fn main() {
    fn test(func: fn(word: String, k: i32) -> i32) {
        assert_eq!(func(String::from("leetcodeleet"), 4), 1);
        assert_eq!(func(String::from("leetcoleet"), 2), 3);
    }
    test(minimum_operations_to_make_k_periodic);
}
