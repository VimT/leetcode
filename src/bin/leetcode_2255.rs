//! 统计是给定字符串前缀的字符串数目

use leetcode::svec;

pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
    words.into_iter().filter(|x| {
        s.starts_with(x)
    }).count() as i32
}

fn main() {
    assert_eq!(count_prefixes(svec!["a","b","c","ab","bc","abc"], String::from("abc")), 3);
    assert_eq!(count_prefixes(svec!["a","a"], String::from("aa")), 2);
}
