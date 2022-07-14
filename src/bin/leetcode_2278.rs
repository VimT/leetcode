//! 字母在字符串中的百分比

pub fn percentage_letter(s: String, letter: char) -> i32 {
    let mut cnt = 0;
    for &ch in s.as_bytes() {
        if ch == letter as u8 {
            cnt += 1;
        }
    }
    (cnt * 100 / s.len()) as i32
}

fn main() {
    fn test(func: fn(s: String, letter: char) -> i32) {
        assert_eq!(func(String::from("foobar"), 'o'), 33);
        assert_eq!(func(String::from("jjjj"), 'k'), 0);
    }
    test(percentage_letter);
}
