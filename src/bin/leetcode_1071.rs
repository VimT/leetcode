//! 字符串的最大公因子

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let len1 = str1.len();
    let len2 = str2.len();
    fn check(s1: &[u8], s2: &[u8], tl: usize) -> bool {
        if s1.len() % tl != 0 || s2.len() % tl != 0 {
            return false;
        }
        let target = &s1[..tl];
        for i in (0..s1.len()).step_by(tl) {
            if &s1[i..i + tl] != target {
                return false;
            }
        }
        for i in (0..s2.len()).step_by(tl) {
            if &s2[i..i + tl] != target {
                return false;
            }
        }
        true
    }
    let s1 = str1.as_bytes();
    let s2 = str2.as_bytes();
    for i in (1..=len1.min(len2)).rev() {
        if check(s1, s2, i) {
            unsafe { return String::from_utf8_unchecked(s1[..i].to_vec()); }
        }
    }
    String::new()
}

fn main() {
    assert_eq!(gcd_of_strings(String::from("ABCABC"), String::from("ABC")), "ABC");
    assert_eq!(gcd_of_strings(String::from("ABABAB"), String::from("ABAB")), "AB");
    assert_eq!(gcd_of_strings(String::from("LEET"), String::from("CODE")), "");
}
