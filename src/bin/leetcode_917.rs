//! 仅仅反转字母

pub fn reverse_only_letters(s: String) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = s.to_vec();
    let mut p = 0;
    while p < len && !result[p].is_ascii_alphabetic() { p += 1; }
    for i in (0..len).rev() {
        if !s[i].is_ascii_alphabetic() { continue; }
        result[p] = s[i];
        p += 1;
        while p < len && !result[p].is_ascii_alphabetic() { p += 1; }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(reverse_only_letters(String::from(";1yDV")), String::from(";1VDy"));
    assert_eq!(reverse_only_letters(String::from("--")), String::from("--"));
    assert_eq!(reverse_only_letters(String::from("ab-cd")), String::from("dc-ba"));
    assert_eq!(reverse_only_letters(String::from("a-bC-dEf-ghIj")), String::from("j-Ih-gfE-dCba"));
    assert_eq!(reverse_only_letters(String::from("Test1ng-Leet=code-Q!")), String::from("Qedo1ct-eeLg=ntse-T!"));
}
