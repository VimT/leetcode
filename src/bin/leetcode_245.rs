//! 最短单词距离 III

use std::collections::HashMap;

use leetcode::svec;

pub fn shortest_word_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
    let mut pos = HashMap::new();
    for (i, word) in words_dict.into_iter().enumerate() {
        pos.entry(word).or_insert(vec![]).push(i);
    }
    if word1 == word2 {
        return pos[&word1].windows(2).map(|x| (x[0] as i32 - x[1] as i32).abs()).min().unwrap();
    }
    let pos1 = &pos[&word1];
    let pos2 = &pos[&word2];
    let mut i1 = 0;
    let mut i2 = 0;
    let mut result = i32::MAX;
    while i1 < pos1.len() && i2 < pos2.len() {
        result = result.min((pos1[i1] as i32 - pos2[i2] as i32).abs());
        if pos1[i1] < pos2[i2] {
            i1 += 1;
        } else {
            i2 += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(words_dict: Vec<String>, word1: String, word2: String) -> i32) {
        assert_eq!(func(svec!["practice", "makes", "perfect", "coding", "makes"], String::from("makes"), String::from("makes")), 3);
        assert_eq!(func(svec!["practice", "makes", "perfect", "coding", "makes"], String::from("makes"), String::from("coding")), 1);
    }
    test(shortest_word_distance);
}
