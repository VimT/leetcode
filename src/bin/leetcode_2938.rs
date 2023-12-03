//! 区分黑球与白球

pub fn minimum_steps(s: String) -> i64 {
    let mut i = 0;
    let s = s.as_bytes();
    let mut result = 0;
    for (j, &ch) in s.iter().enumerate() {
        if ch == b'0' {
            result += j - i;
            i += 1;
        }
    }
    result as i64
}

fn main() {
    fn test(func: fn(s: String) -> i64) {
        assert_eq!(func(String::from("101")), 1);
        assert_eq!(func(String::from("100")), 2);
        assert_eq!(func(String::from("0111")), 0);
    }
    test(minimum_steps);
}
