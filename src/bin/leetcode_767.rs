//! 重构字符串

use std::collections::BinaryHeap;

pub fn reorganize_string(s: String) -> String {
    let mut cnt = [0; 26];
    let s = s.into_bytes();
    for &ch in &s {
        cnt[(ch - b'a') as usize] += 1;
    }
    let len = s.len();
    let mut heap = BinaryHeap::new();
    for i in 0..26 {
        if cnt[i] > 0 {
            if cnt[i] > (len + 1) / 2 {
                return String::new();
            }
            heap.push((cnt[i], i as u8));
        }
    }
    let mut result = vec![0; len];
    for i in 0..len / 2 {
        let (cnt, ch) = heap.pop().unwrap();
        result[i * 2] = ch + b'a';
        let (cnt2, ch2) = heap.pop().unwrap();
        result[i * 2 + 1] = ch2 + b'a';
        if cnt > 1 {
            heap.push((cnt - 1, ch));
        }
        if cnt2 > 1 {
            heap.push((cnt2 - 1, ch2));
        }
    }
    if !heap.is_empty() {
        result[len - 1] = heap.pop().unwrap().1 + b'a';
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(reorganize_string(String::from("ogccckcwmbmxtsbmozli")), String::from("cmcomcbzxwtsomlkigcb"));
    assert_eq!(reorganize_string(String::from("vvvlo")), String::from("vovlv"));
    assert_eq!(reorganize_string(String::from("aab")), String::from("aba"));
    assert_eq!(reorganize_string(String::from("aaab")), String::from(""));
}
