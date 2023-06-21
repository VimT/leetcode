//! 找到最长的半重复子字符串

pub fn longest_semi_repetitive_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut last = 0;
    let mut last2 = 0;
    let mut result = 1;
    for i in 1..len {
        if s[i] == s[i - 1] {
            last2 = last;
            last = i;
        }
        result = result.max(i - last2 + 1);
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("52233")), 4);
        assert_eq!(func(String::from("5494")), 4);
        assert_eq!(func(String::from("1111111")), 2);
        assert_eq!(func(String::from("0")), 1);
    }
    test(longest_semi_repetitive_substring);
}
