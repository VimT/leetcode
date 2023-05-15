//! 字典序最小的美丽字符串

pub fn smallest_beautiful_string(s: String, k: i32) -> String {
    let mut s = s.into_bytes();
    let k = b'a' + k as u8 - 1;
    let len = s.len();
    for i in (0..len).rev() {
        for up in s[i] + 1..=k {
            if (i > 0 && up == s[i - 1]) || (i > 1 && up == s[i - 2]) {
                continue;
            }
            s[i] = up;
            for j in i + 1..len {
                for ch in b'a'..=b'c' {
                    if ch != s[j - 1] && (j <= 1 || ch != s[j - 2]) {
                        s[j] = ch;
                        break;
                    }
                }
            }
            return unsafe { String::from_utf8_unchecked(s) };
        }
    }
    String::new()
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> String) {
        assert_eq!(func(String::from("abdc"), 4), String::from("acba"));
        assert_eq!(func(String::from("abcz"), 26), String::from("abda"));
        assert_eq!(func(String::from("dc"), 4), String::from(""));
    }
    test(smallest_beautiful_string);
}
