//! 统计特殊字母的数量 I

pub fn number_of_special_chars(word: String) -> i32 {
    let s = word.as_bytes();
    let mut a = [false; 26];
    let mut b = [false; 26];
    let mut result = 0;
    for &ch in s {
        if ch.is_ascii_uppercase() {
            b[(ch - b'A') as usize] = true;
        } else if ch.is_ascii_lowercase() {
            a[(ch - b'a') as usize] = true;
        }
    }
    for i in 0..26 {
        if a[i] && b[i] {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(word: String) -> i32) {
        assert_eq!(func(String::from("aaAbcBC")), 3);
        assert_eq!(func(String::from("abc")), 0);
        assert_eq!(func(String::from("abBCab")), 1);
    }
    test(number_of_special_chars);
}
