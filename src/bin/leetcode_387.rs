//! 字符串中的第一个唯一字符

pub fn first_uniq_char(s: String) -> i32 {
    let mut c = vec![0; 26];
    let bytes = s.as_bytes();
    for i in bytes {
        c[(i - b'a') as usize] += 1;
    }

    for (i, v) in bytes.iter().enumerate() {
        if c[(v - b'a') as usize] == 1 {
            return i as i32;
        }
    }
    -1
}


fn main() {
    assert_eq!(first_uniq_char(String::from("leetcode")), 0);
    assert_eq!(first_uniq_char(String::from("loveleetcode")), 2);
    assert_eq!(first_uniq_char(String::from("aabb")), -1);
}
