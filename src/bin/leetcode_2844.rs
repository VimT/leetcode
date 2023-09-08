//! 生成特殊数字的最少操作

pub fn minimum_operations(num: String) -> i32 {
    fn find_min(s: &[u8], a: u8, b: u8) -> i32 {
        if let Some(i) = s.iter().rposition(|&x| x == b) {
            if let Some(j) = s[..i].iter().rposition(|&x| x == a) {
                return (s.len() - j - 2) as i32;
            }
        }
        s.len() as i32
    }
    let s = num.into_bytes();
    let mut result = s.len() as i32;
    if s.iter().find(|&&x| x == b'0').is_some() {
        result -= 1;
    }
    result.min(find_min(&s, b'0', b'0'))
        .min(find_min(&s, b'2', b'5'))
        .min(find_min(&s, b'5', b'0'))
        .min(find_min(&s, b'7', b'5'))
}

fn main() {
    fn test(func: fn(num: String) -> i32) {
        assert_eq!(func(String::from("5")), 1);
        assert_eq!(func(String::from("2245047")), 2);
        assert_eq!(func(String::from("2908305")), 3);
        assert_eq!(func(String::from("10")), 1);
    }
    test(minimum_operations);
}
