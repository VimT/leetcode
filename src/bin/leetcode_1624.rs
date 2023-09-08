//! 两个相同字符之间的最长子字符串

pub fn max_length_between_equal_characters(s: String) -> i32 {
    let mut result = -1;
    let mut last = [None; 26];
    for (i, &ch) in s.as_bytes().iter().enumerate() {
        if let Some(j) = last[(ch - b'a') as usize] {
            result = result.max(i as i32 - j as i32 - 1);
        } else {
            last[(ch - b'a') as usize] = Some(i);
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("aa")), 0);
        assert_eq!(func(String::from("abca")), 2);
        assert_eq!(func(String::from("cbzxy")), -1);
        assert_eq!(func(String::from("cabbac")), 4);
    }
    test(max_length_between_equal_characters);
}
