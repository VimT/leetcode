//! 生成每种字符都是奇数个的字符串

pub fn generate_the_string(n: i32) -> String {
    if n & 1 == 1 {
        "a".repeat(n as usize)
    } else {
        "a".repeat(n as usize - 1) + "b"
    }
}

fn main() {
    fn test(func: fn(n: i32) -> String) {
        assert_eq!(func(4), String::from("aaab"));
        assert_eq!(func(2), String::from("ab"));
        assert_eq!(func(7), String::from("aaaaaaa"));
    }
    test(generate_the_string);
}
