//! 最长的字母序连续子字符串的长度

pub fn longest_continuous_substring(s: String) -> i32 {
    let mut last = 1;
    let s = s.as_bytes();
    let mut result = 1;
    for i in 1..s.len() {
        if s[i] == s[i - 1] + 1 {
            last += 1;
            result = result.max(last);
        } else {
            last = 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("abacaba")), 2);
        assert_eq!(func(String::from("abcde")), 5);
    }
    test(longest_continuous_substring);
}
