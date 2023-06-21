//! 整理字符串

pub fn make_good(s: String) -> String {
    let mut stk = vec![];
    for &ch in s.as_bytes() {
        if !stk.is_empty() {
            if (ch.is_ascii_lowercase() && *stk.last().unwrap() == ch.to_ascii_uppercase()) || (ch.is_ascii_uppercase() && *stk.last().unwrap() == ch.to_ascii_lowercase()) {
                stk.pop();
                continue;
            }
        }
        stk.push(ch);
    }

    unsafe { String::from_utf8_unchecked(stk) }
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("leEeetcode")), String::from("leetcode"));
        assert_eq!(func(String::from("abBAcC")), String::from(""));
        assert_eq!(func(String::from("s")), String::from("s"));
    }
    test(make_good);
}
