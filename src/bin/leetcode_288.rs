//! 单词的唯一缩写

use std::collections::{HashMap, HashSet};
use leetcode::svec;

struct ValidWordAbbr {
    map: HashMap<String, HashSet<String>>,
}

fn to_simple(s: &String) -> String {
    if s.len() <= 2 { return s.clone(); }
    let sb = s.as_bytes();
    format!("{}{}{}", sb[0] as char, sb.len() - 2, sb[sb.len() - 1] as char)
}

impl ValidWordAbbr {
    fn new(dictionary: Vec<String>) -> Self {
        let mut map: HashMap<String, HashSet<String>> = HashMap::new();
        for word in dictionary {
            map.entry(to_simple(&word)).or_default().insert(word);
        }
        Self { map }
    }

    fn is_unique(&self, word: String) -> bool {
        if let Some(word_list) = self.map.get(&to_simple(&word)) {
            if word_list.len() == 1 && word_list.contains(&word) {
                return true;
            }
            return false;
        }
        true
    }
}

fn main() {
    let vw = ValidWordAbbr::new(svec!["deer","door","cake","card"]);
    assert_eq!(vw.is_unique(String::from("dear")), false);
    assert_eq!(vw.is_unique(String::from("cart")), true);
    assert_eq!(vw.is_unique(String::from("cane")), false);
    assert_eq!(vw.is_unique(String::from("make")), true);
    assert_eq!(vw.is_unique(String::from("cake")), true);
    assert_eq!(vw.is_unique(String::from("door")), false);
}
