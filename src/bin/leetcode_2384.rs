//! 最大回文数字

pub fn largest_palindromic(num: String) -> String {
    let mut cnt = [0; 10];
    for &ch in num.as_bytes() {
        cnt[(ch - b'0') as usize] += 1;
    }
    let mut result = Vec::with_capacity(num.len());
    let mut more = None;
    for i in (0..10).rev() {
        if cnt[i] > 0 {
            if i == 0 && result.is_empty() { break; }
            for _ in 0..cnt[i] / 2 {
                result.push(i as u8 + b'0');
            }
            if more.is_none() && cnt[i] & 1 == 1 {
                more = Some(i);
            }
        }
    }
    let end = result.len();
    if let Some(num) = more {
        result.push(num as u8 + b'0');
    }
    for i in (0..end).rev() {
        result.push(result[i]);
    }
    if result.is_empty() {
        result.push(b'0');
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(num: String) -> String) {
        assert_eq!(func(String::from("444947137")), String::from("7449447"));
        assert_eq!(func(String::from("00009")), String::from("9"));
    }
    test(largest_palindromic);
}
