//! 将字符串中的元音字母排序

pub fn sort_vowels(s: String) -> String {
    let mut s = s.into_bytes();
    let mut o = vec![];
    for &ch in &s {
        if matches!(ch.to_ascii_lowercase(), b'a'|b'e'|b'i'|b'o'|b'u') {
            o.push(ch);
        }
    }
    o.sort_unstable();
    let mut i = 0;
    for ch in &mut s {
        if matches!(ch.to_ascii_lowercase(), b'a'|b'e'|b'i'|b'o'|b'u') {
            *ch = o[i];
            i += 1;
        }
    }
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("lEetcOde")), String::from("lEOtcede"));
        assert_eq!(func(String::from("lYmpH")), String::from("lYmpH"));
    }
    test(sort_vowels);
}
