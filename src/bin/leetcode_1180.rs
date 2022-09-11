//! 统计只含单一字母的子串

pub fn count_letters(s: String) -> i32 {
    let s = s.as_bytes();
    let mut cnt = 1;
    let mut cur_ch = s[0];
    let mut result = 1;
    for &ch in &s[1..] {
        if ch == cur_ch {
            cnt += 1;
        } else {
            cnt = 1;
            cur_ch = ch;
        }
        result += cnt;
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("aaaba")), 8);
        assert_eq!(func(String::from("aaaaaaaaaa")), 55);
    }
    test(count_letters);
}
