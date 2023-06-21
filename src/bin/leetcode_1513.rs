//! 仅含 1 的子串数

pub fn num_sub(s: String) -> i32 {
    let s = s.as_bytes();
    let mut result = 0;
    let len = s.len();
    let mut one = 0;
    for r in 0..len {
        if s[r] == b'1' {
            one += 1;
            result = (result + one) % (1e9 as i32 + 7);
        } else {
            one = 0;
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("0110111")), 9);
        assert_eq!(func(String::from("101")), 2);
        assert_eq!(func(String::from("111111")), 21);
        assert_eq!(func(String::from("000")), 0);
    }
    test(num_sub);
}
