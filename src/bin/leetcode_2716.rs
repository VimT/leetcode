//! 最小化字符串长度

pub fn minimized_string_length(s: String) -> i32 {
    let mut cnt = [0; 26];
    for &ch in s.as_bytes() {
        cnt[(ch - b'a') as usize] = 1;
    }
    cnt.iter().sum()
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("aaabc")), 3);
        assert_eq!(func(String::from("cbbd")), 3);
        assert_eq!(func(String::from("dddaaa")), 2);
    }
    test(minimized_string_length);
}
