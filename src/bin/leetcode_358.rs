//! K 距离间隔重排字符串

use std::collections::BinaryHeap;

/// 堆，每次取k个
pub fn rearrange_string(s: String, k: i32) -> String {
    if k == 0 { return s; }
    let mut cnt = [0; 26];
    for &ch in s.as_bytes() {
        cnt[(ch - b'a') as usize] += 1;
    }
    let mut heap = BinaryHeap::new();
    for i in 0..26 {
        if cnt[i] > 0 {
            heap.push((cnt[i], i as u8 + b'a'));
        }
    }
    let mut result = Vec::with_capacity(s.len());
    while result.len() < s.len() {
        let mut tmp = Vec::with_capacity(k as usize);
        for _ in 0..k {
            if result.len() == s.len() { break; }
            if heap.is_empty() {
                return String::new();
            }
            let (cnt, ch) = heap.pop().unwrap();
            result.push(ch);
            if cnt > 1 {
                tmp.push((cnt - 1, ch));
            }
        }
        for i in tmp {
            heap.push(i);
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> String) {
        assert_eq!(func(String::from("abb"), 2), String::from("bab"));
        assert_eq!(func(String::from("aabbcc"), 3), String::from("cbacba"));
        assert_eq!(func(String::from("aaabc"), 3), String::from(""));
        assert_eq!(func(String::from("aaadbbcc"), 2), String::from("acbadcba"));
    }
    test(rearrange_string);
}
