//! 反转单词前缀

pub fn reverse_prefix(mut word: String, ch: char) -> String {
    if let Some(i) = word.find(ch) {
        unsafe { word.as_bytes_mut()[..=i].reverse(); }
    }
    word
}

fn main() {
    assert_eq!(reverse_prefix(String::from("abcdefd"), 'd'), String::from("dcbaefd"));
    assert_eq!(reverse_prefix(String::from("xyxzxe"), 'z'), String::from("zxyxxe"));
    assert_eq!(reverse_prefix(String::from("abcd"), 'z'), String::from("abcd"));
}
