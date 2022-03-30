//! 反转字符串中的元音字母

pub fn reverse_vowels(s: String) -> String {
    let mut s = s.as_bytes().to_vec();
    let mut l = 0;
    let mut r = s.len() - 1;
    #[inline]
    fn is_yuan(ch: u8) -> bool {
        match ch {
            b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => true,
            _ => false
        }
    }
    while l < r {
        while l < r && !is_yuan(s[l]) { l += 1; }
        while l < r && !is_yuan(s[r]) { r -= 1; }
        s.swap(l, r);
        l += 1;
        if r == 0 { break; }
        r -= 1;
    }
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    assert_eq!(reverse_vowels(String::from("aA")), String::from("Aa"));
    assert_eq!(reverse_vowels(String::from("a.")), String::from("a."));
    assert_eq!(reverse_vowels(String::from("hello")), String::from("holle"));
    assert_eq!(reverse_vowels(String::from("leetcode")), String::from("leotcede"));
}
