//! 移除字母异位词后的结果数组

use leetcode::svec;

pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
    let simple_word: Vec<[u8; 26]> = words.iter().map(|x| {
        let mut result = [0; 26];
        for &ch in x.as_bytes() {
            result[(ch - b'a') as usize] += 1;
        }
        result
    }).collect();
    let mut result = vec![words[0].clone()];
    let len = words.len();
    for i in 1..len {
        if simple_word[i] != simple_word[i - 1] {
            result.push(words[i].clone());
        }
    }
    result
}

fn main() {
    fn test(func: fn(words: Vec<String>) -> Vec<String>) {
        assert_eq!(func(svec!["abba","baba","bbaa","cd","cd"]), vec!["abba", "cd"]);
        assert_eq!(func(svec!["a","b","c","d","e"]), vec!["a", "b", "c", "d", "e"]);
    }
    test(remove_anagrams);
}
