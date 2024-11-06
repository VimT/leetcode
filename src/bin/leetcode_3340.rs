//! 检查平衡字符串

pub fn is_balanced(num: String) -> bool {
    num.as_bytes().iter().enumerate().map(|(i, &ch)| {
        if i % 2 == 0 { (ch - b'0') as i32 } else { -((ch - b'0') as i32) }
    }).sum::<i32>() == 0
}

fn main() {
    fn test(func: fn(num: String) -> bool) {
        assert_eq!(func(String::from("1234")), false);
        assert_eq!(func(String::from("24123")), true);
    }
    test(is_balanced);
}
