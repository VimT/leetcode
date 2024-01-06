//! 构造有效字符串的最少插入数

pub fn add_minimum(s: String) -> i32 {
    let len = s.len();
    let mut result = 0;
    let mut i = 0;
    while i < len {
        if i + 3 <= len && &s[i..i + 3] == "abc" {
            i += 3;
        } else if i + 2 <= len && matches!(&s[i..i+2], "ab" | "bc" | "ac") {
            i += 2;
            result += 1;
        } else {
            result += 2;
            i += 1
        }
    }
    result
}

/// 相邻字母x, y 满足 x >= y，说明是两个独立的周期
pub fn add_minimum2(s: String) -> i32 {
    ((s.as_bytes().windows(2).filter(|x| x[0] >= x[1]).count() + 1) * 3 - s.len()) as i32
}

fn main() {
    fn test(func: fn(word: String) -> i32) {
        assert_eq!(func(String::from("aaaaac")), 9);
        assert_eq!(func(String::from("b")), 2);
        assert_eq!(func(String::from("aaa")), 6);
        assert_eq!(func(String::from("abc")), 0);
    }
    test(add_minimum);
    test(add_minimum2);
}
