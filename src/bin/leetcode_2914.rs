//! 使二进制字符串变美丽的最少修改次数

pub fn min_changes(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = 0;
    for i in (0..len).step_by(2) {
        if s[i] != s[i + 1] {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("1001")), 2);
        assert_eq!(func(String::from("10")), 1);
        assert_eq!(func(String::from("0000")), 0);
    }
    test(min_changes);
}
