//! 字符串中的单词数

pub fn count_segments(s: String) -> i32 {
    s.split(" ").filter(|&x| x != "").count() as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("Hello, my name is John")), 5);
    }
    test(count_segments);
}
