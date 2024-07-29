//! 将 1 移动到末尾的最大操作次数

pub fn max_operations(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut i = 0;
    let mut result = 0;
    let mut one = 0;
    while i < len {
        if s[i] == b'1' {
            one += 1;
        } else if i > 0 && s[i - 1] == b'1' {
            result += one;
        }
        i += 1;
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("1001101")), 4);
        assert_eq!(func(String::from("00111")), 0);
    }
    test(max_operations);
}
