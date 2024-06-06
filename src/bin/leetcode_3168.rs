//! 候诊室中的最少椅子数

pub fn minimum_chairs(s: String) -> i32 {
    let mut result = 0;
    let mut count = 0;
    for &ch in s.as_bytes() {
        if ch == b'E' {
            count += 1;
            result = result.max(count);
        } else {
            count -= 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("EEEEEEE")), 7);
        assert_eq!(func(String::from("ELELEEL")), 2);
        assert_eq!(func(String::from("ELEELEELLL")), 3);
    }
    test(minimum_chairs);
}
