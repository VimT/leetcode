//! 第一个出现两次的字母

pub fn repeated_character(s: String) -> char {
    let mut seen = [false; 26];
    for &ch in s.as_bytes() {
        if seen[(ch - b'a') as usize] {
            return ch as char;
        }
        seen[(ch - b'a') as usize] = true;
    }
    unreachable!()
}

fn main() {
    fn test(func: fn(s: String) -> char) {
        assert_eq!(func(String::from("abccbaacz")), 'c');
        assert_eq!(func(String::from("abcdd")), 'd');
    }
    test(repeated_character);
}
