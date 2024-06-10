//! 清除数字

pub fn clear_digits(s: String) -> String {
    let mut stk = vec![];
    for &ch in s.as_bytes() {
        if ch.is_ascii_digit() {
            stk.pop();
        } else {
            stk.push(ch);
        }
    }
    String::from_utf8(stk).unwrap()
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("abc")), String::from("abc"));
        assert_eq!(func(String::from("cb34")), String::from(""));
    }
    test(clear_digits);
}
