//! 故障键盘

use std::collections::VecDeque;

pub fn final_string(s: String) -> String {
    let mut result = vec![];
    for &ch in s.as_bytes() {
        if ch == b'i' {
            result.reverse();
        } else {
            result.push(ch);
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

/// 双端队列，O(n)
pub fn final_string2(s: String) -> String {
    let mut result = VecDeque::new();
    let mut r = false;
    for &ch in s.as_bytes() {
        if ch == b'i' {
            r = !r;
        } else {
            if r { result.push_front(ch) } else { result.push_back(ch) }
        }
    }
    unsafe { String::from_utf8_unchecked(if r { result.into_iter().rev().collect() } else { result.into_iter().collect() }) }
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("string")), String::from("rtsng"));
        assert_eq!(func(String::from("poiinter")), String::from("ponter"));
    }
    test(final_string);
    test(final_string2);
}
