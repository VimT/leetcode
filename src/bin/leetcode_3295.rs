//! 举报垃圾信息

use std::collections::HashSet;

pub fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
    let m: HashSet<String> = banned_words.into_iter().collect();
    message.into_iter().fold(0, |acc, word| acc + m.contains(&word) as i32) > 1
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(message: Vec<String>, banned_words: Vec<String>) -> bool) {
        assert_eq!(func(svec!["hello","world","leetcode"], svec!["world","hello"]), true);
        assert_eq!(func(svec!["hello","programming","fun"], svec!["world","programming","leetcode"]), false);
    }
    test(report_spam);
}
