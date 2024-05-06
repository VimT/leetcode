//! 有效单词

pub fn is_valid(word: String) -> bool {
    let s = word.as_bytes();
    if s.len() < 3 { return false; }
    let mut f = [false; 2];
    for &ch in s {
        if ch.is_ascii_alphabetic() {
            f[(matches!(ch.to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u') as usize)] = true;
        } else if !ch.is_ascii_alphanumeric() {
            return false;
        }
    }
    f[0] && f[1]
}

fn main() {
    fn test(func: fn(word: String) -> bool) {
        assert_eq!(func(String::from("234Adas")), true);
        assert_eq!(func(String::from("b3")), false);
        assert_eq!(func(String::from("a3$e")), false);
    }
    test(is_valid);
}
