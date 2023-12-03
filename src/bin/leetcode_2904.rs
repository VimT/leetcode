//! 最短且字典序最小的美丽子字符串

pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let mut l = 0;
    let mut one = 0;
    let mut result = &s[len..];
    for r in 0..len {
        if s[r] == b'1' { one += 1; }
        while one > k {
            if s[l] == b'1' { one -= 1; }
            l += 1;
        }
        while l < r && s[l] == b'0' { l += 1; }
        if one == k && (result.is_empty() || (r + 1 - l, &s[l..=r]) < (result.len(), result)) {
            result = &s[l..=r];
        }
    }
    unsafe { String::from_utf8_unchecked(result.to_vec()) }
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> String) {
        assert_eq!(func(String::from("100011001"), 3), String::from("11001"));
        assert_eq!(func(String::from("1011"), 2), String::from("11"));
        assert_eq!(func(String::from("000"), 1), String::from(""));
    }
    test(shortest_beautiful_substring);
}
