//! 统计追加字母可以获得的单词数

use std::collections::HashSet;

use leetcode::svec;

pub fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
    let mut start_set = HashSet::new();
    for word in start_words {
        let mut num = 0;
        for &ch in word.as_bytes() {
            num |= 1 << (ch - b'a');
        }
        start_set.insert(num);
    }
    let mut result = 0;
    for word in target_words {
        let mut num = 0;
        for &ch in word.as_bytes() {
            num |= 1 << (ch - b'a');
        }
        for &ch in word.as_bytes() {
            if start_set.contains(&(num ^ (1 << (ch - b'a')))) {
                result += 1;
                break;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(word_count(svec!["ant", "act", "tack"], svec!["tack", "act", "acti"]), 2);
    assert_eq!(word_count(svec!["ab", "a"], svec!["abc", "abcd"]), 1);
}
