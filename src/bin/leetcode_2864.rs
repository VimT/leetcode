//! 最大二进制奇数

pub fn maximum_odd_binary_number(s: String) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let cnt = s.iter().filter(|x| **x == b'1').count();
    let mut result = vec![b'0'; len];
    result[len - 1] = b'1';
    for i in 0..cnt - 1 {
        result[i] = b'1';
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("010")), String::from("001"));
        assert_eq!(func(String::from("0101")), String::from("1001"));
    }
    test(maximum_odd_binary_number);
}
