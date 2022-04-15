//! 拼写单词

use leetcode::svec;

pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut cnt = [0; 26];
    for &ch in chars.as_bytes() {
        cnt[(ch - b'a') as usize] += 1;
    }
    words.into_iter().map(|word| {
        let mut wc = [0; 26];
        for &ch in word.as_bytes() {
            wc[(ch - b'a') as usize] += 1;
        }
        for i in 0..26 {
            if cnt[i] < wc[i] {
                return 0;
            }
        }
        word.len() as i32
    }).sum()
}

fn main() {
    fn test(func: fn(words: Vec<String>, chars: String) -> i32) {
        assert_eq!(func(svec!["cat","bt","hat","tree"], String::from("atach")), 6);
        assert_eq!(func(svec!["hello","world","leetcode"], String::from("welldonehoneyr")), 10);
    }
    test(count_characters);
}
