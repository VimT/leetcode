//! 字典序最小回文串

pub fn make_smallest_palindrome(s: String) -> String {
    let mut s = s.into_bytes();
    let len = s.len();
    for i in 0..len / 2 {
        if s[i] < s[len - 1 - i] {
            s[len - 1 - i] = s[i];
        } else {
            s[i] = s[len - 1 - i];
        }
    }
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("egcfe")), String::from("efcfe"));
        assert_eq!(func(String::from("abcd")), String::from("abba"));
        assert_eq!(func(String::from("seven")), String::from("neven"));
    }
    test(make_smallest_palindrome);
}
