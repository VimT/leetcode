//! 统计以给定字符开头和结尾的子字符串总数

pub fn count_substrings(s: String, c: char) -> i64 {
    let cnt = s.chars().filter(|&x| x == c).count() as i64;
    cnt * (cnt + 1) / 2
}

fn main() {
    fn test(func: fn(s: String, c: char) -> i64) {
        assert_eq!(func(String::from("zzzz"), 'z'), 10);
        assert_eq!(func(String::from("zz"), 'z'), 3);
        assert_eq!(func(String::from("abada"), 'a'), 6);
        assert_eq!(func(String::from("zzz"), 'z'), 6);
    }
    test(count_substrings);
}
