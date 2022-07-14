//! Excel表列名称

pub fn convert_to_title(mut n: i32) -> String {
    let mut result = Vec::new();
    while n != 0 {
        n = n - 1;
        result.push((n % 26) as u8 + b'A');
        n = n / 26;
    }
    result.reverse();
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(column_number: i32) -> String) {
        assert_eq!(func(1), String::from("A"));
        assert_eq!(func(28), String::from("AB"));
        assert_eq!(func(701), String::from("ZY"));
    }
    test(convert_to_title);
}
