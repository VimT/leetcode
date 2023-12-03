//! 统计美丽子字符串 I

pub fn beautiful_substrings(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = 0;
    for i in 0..len {
        let mut a = 0;
        let mut b = 0;
        for j in i..len {
            if matches!(s[j], b'a' | b'e' | b'i' | b'o' | b'u') {
                a += 1;
            } else {
                b += 1;
            }
            if a == b && (a * b) % k == 0 {
                result += 1;
            }
        }
    }

    result
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("baeyh"), 2), 2);
        assert_eq!(func(String::from("abba"), 1), 3);
        assert_eq!(func(String::from("bcdf"), 1), 0);
    }
    test(beautiful_substrings);
}
