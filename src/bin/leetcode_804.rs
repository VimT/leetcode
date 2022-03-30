//! 唯一摩尔斯密码词

use std::collections::HashSet;

use leetcode::svec;

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    static MOSI: [&str; 26] = [".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."];
    let mut set = HashSet::new();
    for word in words {
        let mut s = Vec::with_capacity(word.len() * 4);
        for &ch in word.as_bytes() {
            s.extend_from_slice(&MOSI[(ch - b'a') as usize].as_bytes());
        }
        set.insert(s);
    }
    set.len() as i32
}

fn main() {
    assert_eq!(unique_morse_representations(svec!["gin", "zen", "gig", "msg"]), 2);
    assert_eq!(unique_morse_representations(svec!["a"]), 1);
}
