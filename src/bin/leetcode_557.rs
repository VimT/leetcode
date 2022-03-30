//! 反转字符串中的单词 III

pub fn reverse_words(s: String) -> String {
    let mut s = s.as_bytes().to_vec();
    let len = s.len();
    let mut i = 0;
    while i < len {
        let mut j = i + 1;
        while j < len && s[j] != b' ' { j += 1; }
        s[i..j].reverse();
        i = j + 1;
    }
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    assert_eq!(reverse_words(String::from("Let's take LeetCode contest")), String::from("s'teL ekat edoCteeL tsetnoc"));
    assert_eq!(reverse_words(String::from("God Ding")), String::from("doG gniD"));
}
