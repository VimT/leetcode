//! 最短单词距离 II

use std::collections::HashMap;
use leetcode::svec;

struct WordDistance {
    pos: HashMap<String, Vec<usize>>,
}

impl WordDistance {
    fn new(words_dict: Vec<String>) -> Self {
        let mut pos = HashMap::new();
        for (i, word) in words_dict.into_iter().enumerate() {
            pos.entry(word).or_insert(vec![]).push(i);
        }
        Self { pos }
    }

    fn shortest(&self, word1: String, word2: String) -> i32 {
        let pos1 = &self.pos[&word1];
        let pos2 = &self.pos[&word2];
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
}


fn main() {
    let wd = WordDistance::new(svec!["practice", "makes", "perfect", "coding", "makes"]);
    assert_eq!(wd.shortest(String::from("coding"), String::from("practice")), 3);
    assert_eq!(wd.shortest(String::from("coding"), String::from("makes")), 1);
}
