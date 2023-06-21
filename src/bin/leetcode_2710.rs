//! 移除字符串中的尾随零

pub fn remove_trailing_zeros(num: String) -> String {
    num.trim_end_matches('0').to_string()
}

fn main() {
    fn test(func: fn(num: String) -> String) {
        assert_eq!(func(String::from("51230100")), String::from("512301"));
        assert_eq!(func(String::from("123")), String::from("123"));
    }
    test(remove_trailing_zeros);
}
