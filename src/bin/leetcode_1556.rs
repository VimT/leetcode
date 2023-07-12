//! 千位分隔数

pub fn thousand_separator(mut n: i32) -> String {
    let mut result = vec![];
    let mut num = 0;
    while n > 0 {
        if num > 0 && num % 3 == 0 {
            result.push(b'.');
        }
        result.push((n % 10) as u8 + b'0');
        num += 1;
        n /= 10;
    }
    result.reverse();
    if result.is_empty() { result.push(b'0'); }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(n: i32) -> String) {
        assert_eq!(func(987), String::from("987"));
        assert_eq!(func(1234), String::from("1.234"));
        assert_eq!(func(123456789), String::from("123.456.789"));
        assert_eq!(func(0), String::from("0"));
    }
    test(thousand_separator);
}
