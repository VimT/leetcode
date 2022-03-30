//! 反转字符串 II

pub fn reverse_str(s: String, k: i32) -> String {
    let mut s = s.as_bytes().to_vec();
    let len = s.len();
    let k = k as usize;
    for i in (0..s.len()).step_by(2 * k) {
        s[i..(i + k).min(len)].reverse();
    }
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    assert_eq!(reverse_str(String::from("abcdefg"), 3), String::from("cbadefg"));
    assert_eq!(reverse_str(String::from("abcdefg"), 2), String::from("bacdfeg"));
    assert_eq!(reverse_str(String::from("abcd"), 2), String::from("bacd"));
}
