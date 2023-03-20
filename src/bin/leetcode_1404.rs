//! 将二进制表示减到 1 的步骤数

pub fn num_steps(s: String) -> i32 {
    let mut s = s.into_bytes();
    let mut result = 0;
    while s.len() > 1 {
        if *s.last().unwrap() == b'0' {
            result += 1;
            s.pop();
        } else {
            while !s.is_empty() && *s.last().unwrap() == b'1' {
                result += 1;
                s.pop();
            }
            if let Some(a) = s.last_mut() {
                *a += 1;
            }
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("11001")), 8);
        assert_eq!(func(String::from("1101")), 6);
        assert_eq!(func(String::from("10")), 1);
        assert_eq!(func(String::from("1")), 0);
        assert_eq!(func(String::from("11")), 3);
    }
    test(num_steps);
}
