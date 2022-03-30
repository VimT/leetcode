//! 颠倒字符串中的单词

pub fn reverse_words(s: String) -> String {
    s.split_ascii_whitespace().rev().collect::<Vec<_>>().join(" ")
}

fn main() {
    assert_eq!(reverse_words(String::from("the sky is blue")), String::from("blue is sky the"));
    assert_eq!(reverse_words(String::from("  hello world  ")), String::from("world hello"));
    assert_eq!(reverse_words(String::from("a good   example")), String::from("example good a"));
}
