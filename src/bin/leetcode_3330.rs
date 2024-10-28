//! 找到初始输入字符串 I

pub fn possible_string_count(word: String) -> i32 {
    let s = word.as_bytes();
    s.windows(2).filter(|w| w[0] == w[1]).count() as i32 + 1
}

fn main() {
    fn test(func: fn(word: String) -> i32) {
        assert_eq!(func(String::from("abbcccc")), 5);
        assert_eq!(func(String::from("abcd")), 1);
        assert_eq!(func(String::from("aaaa")), 4);
    }
    test(possible_string_count);
}
