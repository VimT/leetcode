//! 最常见的单词

use std::collections::HashMap;

use leetcode::svec;

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let mut cnt = HashMap::new();
    for word in paragraph.split(|x| matches!(x, '!'|'?'|'\''|','|';'|'.'|' ')) {
        if word.len() > 0 {
            *cnt.entry(word.to_ascii_lowercase()).or_insert(0i32) += 1;
        }
    }
    for ban in banned {
        cnt.insert(ban.to_ascii_lowercase(), 0);
    }
    cnt.keys().max_by_key(|x| cnt[*x]).unwrap().to_string()
}

fn main() {
    assert_eq!(most_common_word(String::from("Bob hit a ball, the hit ball flew far after it was hit."), svec!["hit"]), String::from("ball"));
    assert_eq!(most_common_word(String::from("a."), vec![]), String::from("a"));
}
