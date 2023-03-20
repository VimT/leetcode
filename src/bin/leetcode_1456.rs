//! 定长子串中元音的最大数目

pub fn max_vowels(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let k = k as usize;
    let mut cnt = 0;
    for i in 0..k {
        if matches!(s[i], b'a'|b'e'|b'i'|b'o'|b'u') {
            cnt += 1;
        }
    }
    let mut result = cnt;
    for i in k..len {
        if matches!(s[i], b'a'|b'e'|b'i'|b'o'|b'u') {
            cnt += 1;
        }
        if matches!(s[i-k], b'a'|b'e'|b'i'|b'o'|b'u') {
            cnt -= 1;
        }
        result = result.max(cnt);
    }
    result
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("abciiidef"), 3), 3);
        assert_eq!(func(String::from("aeiou"), 2), 2);
        assert_eq!(func(String::from("leetcode"), 3), 2);
    }
    test(max_vowels);
}
