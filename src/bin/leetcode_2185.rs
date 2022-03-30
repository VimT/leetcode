//! 统计包含给定前缀的字符串

use leetcode::svec;

pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    words.into_iter().filter(|x| x.starts_with(&pref)).count() as i32
}

fn main() {
    assert_eq!(prefix_count(svec!["pay", "attention", "practice", "attend"], String::from("at")), 2);
    assert_eq!(prefix_count(svec!["leetcode", "win", "loops", "success"], String::from("code")), 0);
}
