//! 验证外星语词典

use leetcode::svec;

pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let mut m = [0; 26];
    for (i, &ch) in order.as_bytes().iter().enumerate() {
        m[(ch - b'a') as usize] = i as u8;
    }
    let words_conv: Vec<Vec<u8>> = words.iter().map(|x| x.as_bytes().iter().map(|x| m[(*x - b'a') as usize]).collect::<Vec<u8>>()).collect();
    for i in 1..words_conv.len() {
        if words_conv[i] < words_conv[i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    assert_eq!(is_alien_sorted(svec!["hello", "leetcode"], String::from("hlabcdefgijkmnopqrstuvwxyz")), true);
    assert_eq!(is_alien_sorted(svec!["word", "world", "row"], String::from("worldabcefghijkmnpqstuvxyz")), false);
    assert_eq!(is_alien_sorted(svec!["apple", "app"], String::from("abcdefghijklmnopqrstuvwxyz")), false);
}
