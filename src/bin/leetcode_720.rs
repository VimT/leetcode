//! 词典中最长的单词

use std::collections::HashSet;

use leetcode::svec;

pub fn longest_word(mut words: Vec<String>) -> String {
    words.sort_unstable_by_key(|x| x.len());
    let mut set = HashSet::new();
    set.insert("");
    let mut result = String::new();
    for w in &words {
        let s = w.as_bytes();
        unsafe {
            if set.contains(std::str::from_utf8_unchecked(&s[..s.len() - 1])) {
                if (w.len() > result.len()) || (w.len() == result.len() && w < &result) {
                    result = w.to_string();
                }
                set.insert(w.as_str());
            }
        }
    }
    result
}

fn main() {
    assert_eq!(longest_word(svec!["yo", "ew", "fc", "zrc", "yodn", "fcm", "qm", "qmo", "fcmz", "z", "ewq", "yod", "ewqz", "y"]), String::from("yodn"));
    assert_eq!(longest_word(svec!["w", "wo", "wor", "worl", "world"]), String::from("world"));
    assert_eq!(longest_word(svec!["a", "banana", "app", "appl", "ap", "apply", "apple"]), String::from("apple"));
}
