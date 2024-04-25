//! 字符串的分数

pub fn score_of_string(s: String) -> i32 {
    let s = s.as_bytes();
    s.windows(2).map(|x| x[0].abs_diff(x[1]) as i32).sum()
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("hello")), 13);
        assert_eq!(func(String::from("zaz")), 50);
    }
    test(score_of_string);
}
