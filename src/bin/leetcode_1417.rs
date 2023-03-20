//! 重新格式化字符串

pub fn reformat(s: String) -> String {
    let s = s.as_bytes();
    let alpha: Vec<u8> = s.iter().filter(|x| x.is_ascii_alphabetic()).cloned().collect();
    let digit: Vec<u8> = s.iter().filter(|x| x.is_ascii_digit()).cloned().collect();
    if (alpha.len() as i32 - digit.len() as i32).abs() > 1 {
        return String::new();
    }
    let (a, b) = if alpha.len() > digit.len() { (alpha, digit) } else { (digit, alpha) };
    let mut result = vec![0; s.len()];
    for (i, ch) in a.into_iter().enumerate() {
        result[i * 2] = ch;
    }
    for (i, ch) in b.into_iter().enumerate() {
        result[i * 2 + 1] = ch;
    }

    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("a0b1c2")), String::from("0a1b2c"));
        assert_eq!(func(String::from("leetcode")), String::from(""));
        assert_eq!(func(String::from("1229857369")), String::from(""));
    }
    test(reformat);
}
