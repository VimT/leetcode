//! 使两字符串互为字母异位词的最少步骤数

pub fn min_steps(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut scnt = [0i32; 26];
    let mut tcnt = [0i32; 26];
    for &ch in s {
        scnt[(ch - b'a') as usize] += 1;
    }
    for &ch in t {
        tcnt[(ch - b'a') as usize] += 1;
    }
    let mut result = 0;
    for i in 0..26 {
        result += (scnt[i] - tcnt[i]).abs();
    }
    result
}

fn main() {
    assert_eq!(min_steps(String::from("leetcode"), String::from("coats")), 7);
    assert_eq!(min_steps(String::from("night"), String::from("thing")), 0);
}
