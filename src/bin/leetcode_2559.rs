//! 统计范围内的元音字符串数

pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
    words[left as usize..=right as usize].iter().filter(|x| {
        let s = x.as_bytes();
        let len = s.len();
        matches!(s[0], b'a'|b'e'|b'i'|b'o'|b'u') && matches!(s[len-1], b'a'|b'e'|b'i'|b'o'|b'u')
    }).count() as i32
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>, left: i32, right: i32) -> i32) {
        assert_eq!(func(svec!["are","amy","u"], 0, 2), 2);
        assert_eq!(func(svec!["hey","aeo","mu","ooo","artro"], 1, 4), 3);
    }
    test(vowel_strings);
}
