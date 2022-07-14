//! 有效单词缩写

pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
    let w = word.as_bytes();
    let mut i = 0;
    let mut num = 0;
    for &ch in abbr.as_bytes() {
        if ch.is_ascii_alphabetic() {
            i += num;
            num = 0;
            if i >= w.len() || ch != w[i] {
                return false;
            }
            i += 1;
        } else {
            if num == 0 && ch == b'0' { return false; }
            num = num * 10 + (ch - b'0') as usize;
        }
    }
    i + num == w.len()
}

fn main() {
    fn test(func: fn(word: String, abbr: String) -> bool) {
        assert_eq!(func(String::from("internationalization"), String::from("i12iz4n")), true);
        assert_eq!(func(String::from("apple"), String::from("a2e")), false);
    }
    test(valid_word_abbreviation);
}
