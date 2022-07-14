//! 句子相似性

use std::collections::HashSet;
use leetcode::svec;

pub fn are_sentences_similar(sentence1: Vec<String>, sentence2: Vec<String>, similar_pairs: Vec<Vec<String>>) -> bool {
    let map: HashSet<(String, String)> = similar_pairs.into_iter().map(|x| (x[0].clone(), x[1].clone())).collect();
    if sentence1.len() != sentence2.len() { return false; }
    for (a, b) in sentence1.into_iter().zip(sentence2) {
        if a != b && !map.contains(&(a.clone(), b.clone())) && !map.contains(&(b, a)) {
            return false;
        }
    }
    true
}

fn main() {
    fn test(func: fn(sentence1: Vec<String>, sentence2: Vec<String>, similar_pairs: Vec<Vec<String>>) -> bool) {
        assert_eq!(func(svec!["great","acting","skills"], svec!["fine","drama","talent"], vec![svec!["great","fine"], svec!["drama","acting"], svec!["skills","talent"]]), true);
        assert_eq!(func(svec!["great"], svec!["great"], vec![]), true);
        assert_eq!(func(svec!["great"], svec!["doubleplus","good"], vec![svec!["great","doubleplus"]]), false);
    }
    test(are_sentences_similar);
}
