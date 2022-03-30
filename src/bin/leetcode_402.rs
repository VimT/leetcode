//! 移掉 K 位数字

use std::collections::VecDeque;

/// 获取指定长度的最小子序列，单调栈
pub fn remove_kdigits(num: String, k: i32) -> String {
    let s = num.as_bytes();
    let k = k as usize;
    if k >= s.len() { return String::from("0"); }
    let len = s.len();
    let mut result = VecDeque::with_capacity(len - k);
    let mut remain = k;
    for i in 0..len {
        while !result.is_empty() && *result.back().unwrap() > s[i] && remain > 0 {
            result.pop_back().unwrap();
            remain -= 1;
        }
        if result.len() < len - k {
            result.push_back(s[i]);
        } else {
            remain -= 1;
        }
    }
    while !result.is_empty() && *result.front().unwrap() == b'0' {
        result.pop_front().unwrap();
    }
    if result.is_empty() {
        result.push_back(b'0');
    }
    unsafe { String::from_utf8_unchecked(Vec::from(result)) }
}

fn main() {
    assert_eq!(remove_kdigits(String::from("1432219"), 3), String::from("1219"));
    assert_eq!(remove_kdigits(String::from("10200"), 1), String::from("200"));
    assert_eq!(remove_kdigits(String::from("10"), 2), String::from("0"));
    assert_eq!(remove_kdigits(String::from("0"), 1), String::from("0"));
}
