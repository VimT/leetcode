//! 字符串及其反转中是否存在同一子字符串

pub fn is_substring_present(s: String) -> bool {
    let s = s.into_bytes();
    for sub in s.windows(2) {
        for i in 0..s.len() - 1 {
            if s[i] == sub[1] && s[i + 1] == sub[0] {
                return true;
            }
        }
    }
    false
}

fn main() {
    fn test(func: fn(s: String) -> bool) {
        assert_eq!(func(String::from("leetcode")), true);
        assert_eq!(func(String::from("abcba")), true);
        assert_eq!(func(String::from("abcd")), false);
    }
    test(is_substring_present);
}
