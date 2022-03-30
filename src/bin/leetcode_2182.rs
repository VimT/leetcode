//! 构造限制重复的字符串

use std::collections::BinaryHeap;

pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    let s = s.as_bytes();
    let mut cnt = [0; 26];
    for &ch in s {
        cnt[(ch - b'a') as usize] += 1;
    }
    let mut heap = BinaryHeap::new();
    for i in 0..26 {
        if cnt[i] > 0 {
            heap.push((i as u8 + b'a', cnt[i]));
        }
    }
    let mut result = Vec::with_capacity(s.len());
    while !heap.is_empty() {
        let (ch, mut cnt) = heap.pop().unwrap();
        if !result.is_empty() && ch == *result.last().unwrap() {
            if heap.is_empty() {
                break;
            }
            let mut cur = heap.peek_mut().unwrap();
            result.push(cur.0);
            cur.1 -= 1;
            if cur.1 == 0 {
                drop(cur);
                heap.pop().unwrap();
            }
        }
        for _ in 0..cnt.min(repeat_limit) {
            result.push(ch);
            cnt -= 1;
        }
        if cnt > 0 {
            heap.push((ch, cnt));
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}


fn main() {
    assert_eq!(repeat_limited_string(String::from("cczazcc"), 3), String::from("zzcccac"));
    assert_eq!(repeat_limited_string(String::from("aababab"), 2), String::from("bbabaa"));
}
