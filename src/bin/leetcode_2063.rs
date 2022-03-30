//! 所有子字符串中的元音

pub fn count_vowels(word: String) -> i64 {
    let s = word.as_bytes();
    let len = s.len();
    let mut result = 0;
    fn cal(a: i64) -> i64 {
        a * (a + 1) / 2
    }
    let total = cal(len as i64);
    for i in 0..len {
        match s[i] {
            b'a' | b'e' | b'i' | b'o' | b'u' => {
                result += total - cal(i as i64) - cal((len - i - 1) as i64);
            }
            _ => ()
        }
    }
    result
}


fn main() {
    assert_eq!(count_vowels(String::from("aba")), 6);
    assert_eq!(count_vowels(String::from("abc")), 3);
    assert_eq!(count_vowels(String::from("ltcd")), 0);
    assert_eq!(count_vowels(String::from("noosabasboosa")), 237);
}