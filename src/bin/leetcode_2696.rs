//! 删除子串后的字符串最小长度

pub fn min_length(s: String) -> i32 {
    let mut stk = vec![];
    for &ch in s.as_bytes() {
        if ch == b'B' && !stk.is_empty() && *stk.last().unwrap() == b'A' {
            stk.pop();
        } else if ch == b'D' && !stk.is_empty() && *stk.last().unwrap() == b'C' {
            stk.pop();
        } else {
            stk.push(ch);
        }
    }
    stk.len() as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("ABFCACDB")), 2);
        assert_eq!(func(String::from("ACBBD")), 5);
    }
    test(min_length);
}
