//! 前K个高频单词

use std::collections::{BinaryHeap, HashMap};

use leetcode::svec;

pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    let mut m: HashMap<String, i32> = HashMap::new();
    for word in words {
        *m.entry(word).or_default() += 1;
    }
    let mut heap = BinaryHeap::new();
    for (key, v) in m {
        heap.push((-v, key));
        if heap.len() > k as usize {
            heap.pop().unwrap();
        }
    }
    let mut result = Vec::with_capacity(k as usize);
    for _ in 0..k {
        result.push(heap.pop().unwrap().1);
    }
    result.reverse();
    result
}

fn main() {
    assert_eq!(top_k_frequent(svec!["i", "love", "leetcode", "i", "love", "coding"], 2), svec!["i", "love"]);
    assert_eq!(top_k_frequent(svec!["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"], 4), svec!["the", "is", "sunny", "day"]);
}
