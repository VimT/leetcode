//! 移除指定数字得到的最大结果

pub fn remove_digit(number: String, digit: char) -> String {
    let s = number.into_bytes();
    let mut result = vec![];
    for i in (0..s.len()).rev() {
        if s[i] == digit as u8 {
            result = result.max(s[..i].iter().chain(s[i + 1..].iter()).cloned().collect());
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(remove_digit(String::from("123"), '3'), String::from("12"));
    assert_eq!(remove_digit(String::from("1231"), '1'), String::from("231"));
    assert_eq!(remove_digit(String::from("551"), '5'), String::from("51"));
}
