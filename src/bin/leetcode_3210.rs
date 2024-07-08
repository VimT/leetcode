//! 找出加密后的字符串

pub fn get_encrypted_string(s: String, k: i32) -> String {
    let len = s.len();
    let s = s.as_bytes();
    let mut new = vec![0; len];
    let k = k as usize;
    for i in 0..len {
        new[i] = s[(i + k) % len];
    }
    unsafe { String::from_utf8_unchecked(new) }
}

/// 简单写法
pub fn get_encrypted_string2(s: String, k: i32) -> String {
    let k = k as usize % s.len();
    s[k..].to_string() + &s[..k]
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> String) {
        assert_eq!(func(String::from("dart"), 3), String::from("tdar"));
        assert_eq!(func(String::from("aaa"), 1), String::from("aaa"));
    }
    test(get_encrypted_string);
    test(get_encrypted_string2);
}
