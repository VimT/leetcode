//! 元音拼写检查器

use std::collections::HashMap;

use leetcode::svec;

pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    let mut m = HashMap::new();
    let mut ori = HashMap::new();
    let mut yuan = HashMap::new();

    for word in wordlist.into_iter().rev() {
        m.insert(word.to_ascii_lowercase(), word.clone());
        ori.insert(word.clone(), word.clone());
        let mut small = word.to_ascii_lowercase();
        let s = unsafe { small.as_bytes_mut() };
        for ch in s {
            if matches!(*ch, b'a' | b'e' | b'i' | b'o' | b'u') {
                *ch = b'*';
            }
        }
        yuan.insert(small, word);
    }
    queries.into_iter().map(|x| {
        if let Some(v) = ori.get(&x) {
            return v.clone();
        }
        let mut small = x.to_ascii_lowercase();
        if let Some(v) = m.get(&small) {
            return v.clone();
        }
        let s = unsafe { small.as_bytes_mut() };
        for ch in s {
            if matches!(*ch, b'a' | b'e' | b'i' | b'o' | b'u') {
                *ch = b'*';
            }
        }
        if let Some(v) = yuan.get(&small) {
            return v.clone();
        }
        String::new()
    }).collect()
}

fn main() {
    assert_eq!(spellchecker(svec!["ae", "aa"], svec!["UU"]), svec!["ae"]);
    assert_eq!(spellchecker(svec!["KiTe", "kite", "hare", "Hare"], svec!["kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto"]), svec!["kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe"]);
    assert_eq!(spellchecker(svec!["yellow"], svec!["YellOw"]), svec!["yellow"]);
}
