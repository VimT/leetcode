//! 七进制数

pub fn convert_to_base7(mut num: i32) -> String {
    if num == 0 { return "0".to_string(); }
    let mut result = vec![];
    let neg = num < 0;
    if neg { num = -num; }
    while num > 0 {
        result.push((num % 7) as u8 + b'0');
        num /= 7;
    }
    if neg { result.push(b'-'); }
    result.reverse();
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(convert_to_base7(0), String::from("0"));
    assert_eq!(convert_to_base7(100), String::from("202"));
    assert_eq!(convert_to_base7(-7), String::from("-10"));
}
