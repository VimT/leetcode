//! 将字符串拆分为若干长度为 k 的组

use leetcode::svec;

pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let s = s.into_bytes();
    let mut result: Vec<String> = s.chunks(k as usize).map(|x| {
        unsafe { String::from_utf8_unchecked(x.to_vec()) }
    }).collect();
    if let Some(last) = result.last_mut() {
        while last.len() < k as usize {
            last.push(fill);
        }
    }
    result
}

fn main() {
    assert_eq!(divide_string(String::from("abcdefghi"), 3, 'x'), svec!["abc", "def", "ghi"]);
    assert_eq!(divide_string(String::from("abcdefghij"), 3, 'x'), svec!["abc", "def", "ghi", "jxx"]);
}
