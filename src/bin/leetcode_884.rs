//! 两句话中的不常见单词

use std::collections::HashMap;

use leetcode::svec;

pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut set = HashMap::new();
    for i in s1.split(' ') {
        *set.entry(i).or_insert(0i32) += 1;
    }
    for i in s2.split(' ') {
        *set.entry(i).or_insert(0i32) += 1;
    }
    set.iter().filter_map(|(k, v)| {
        if *v == 1 {
            return Some(k.to_string());
        }
        None
    }).collect()
}

fn main() {
    assert_eq!(uncommon_from_sentences(String::from("this apple is sweet"), String::from("this apple is sour")), svec!["sweet", "sour"]);
    assert_eq!(uncommon_from_sentences(String::from("apple apple"), String::from("banana")), svec!["banana"]);
}
