//! 执行逐位运算使字符串相等

pub fn make_strings_equal(s: String, target: String) -> bool {
    s.find('1').is_some() == target.find('1').is_some()
}

fn main() {
    fn test(func: fn(s: String, target: String) -> bool) {
        assert_eq!(func(String::from("001000"), String::from("000100")), true);
        assert_eq!(func(String::from("1010"), String::from("0110")), true);
        assert_eq!(func(String::from("11"), String::from("00")), false);
    }
    test(make_strings_equal);
}
