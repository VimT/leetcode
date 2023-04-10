//! 最长平衡子字符串

pub fn find_the_longest_balanced_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = 0;
    for i in 0..len {
        let mut zero = 0;
        let mut j = i;
        while j < len {
            if s[j] == b'0' {
                zero += 1;
            } else { break; }
            j += 1;
        }
        let mut one = 0;
        while j < len {
            if s[j] == b'1' {
                one += 1;
            } else { break; }
            j += 1;
        }
        result = result.max(one.min(zero) * 2);
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("01000111")), 6);
        assert_eq!(func(String::from("00111")), 4);
        assert_eq!(func(String::from("111")), 0);
    }
    test(find_the_longest_balanced_substring);
}
