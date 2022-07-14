//! 单词规律

use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
    let p = pattern.as_bytes();
    let mut c2s = HashMap::new();
    let mut s2c = HashMap::new();
    let mut len = 0;
    for (i, word) in s.split(' ').enumerate() {
        if i >= p.len() { return false; }
        if let Some(ch_word) = c2s.get(&p[i]) {
            if *ch_word != word { return false; }
        }
        if let Some(ch) = s2c.get(&word) {
            if *ch != p[i] { return false; }
        }
        s2c.insert(word, p[i]);
        c2s.insert(p[i], word);
        len += 1;
    }
    len == p.len()
}

fn main() {
    fn test(func: fn(pattern: String, s: String) -> bool) {
        assert_eq!(func(String::from("aaa"), String::from("aa aa aa aa")), false);
        assert_eq!(func(String::from("abba"), String::from("dog dog dog dog")), false);
        assert_eq!(func(String::from("abba"), String::from("dog cat cat dog")), true);
        assert_eq!(func(String::from("abba"), String::from("dog cat cat fish")), false);
        assert_eq!(func(String::from("aaaa"), String::from("dog cat cat dog")), false);
    }
    test(word_pattern);
}
