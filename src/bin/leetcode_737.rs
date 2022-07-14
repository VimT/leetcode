//! 句子相似性 II

use std::collections::HashMap;
use leetcode::svec;

struct UnionSet {
    f: Vec<usize>,
    size: Vec<usize>,
}

impl UnionSet {
    fn new(n: usize) -> Self {
        UnionSet {
            f: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        return if self.f[x] == x {
            x
        } else {
            self.f[x] = self.find(self.f[x]);
            self.f[x]
        };
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut xx = self.find(x);
        let mut yy = self.find(y);
        if xx == yy { return; }
        if self.size[xx] > self.size[yy] {
            std::mem::swap(&mut xx, &mut yy);
        }
        self.f[xx] = yy;
        self.size[yy] += self.size[xx];
    }
}

pub fn are_sentences_similar_two(sentence1: Vec<String>, sentence2: Vec<String>, similar_pairs: Vec<Vec<String>>) -> bool {
    if sentence1.len() != sentence2.len() { return false; }
    let mut cnt = 0;
    let mut words: HashMap<String, usize> = HashMap::new();
    for pair in &similar_pairs {
        if !words.contains_key(&pair[0]) {
            words.insert(pair[0].clone(), cnt);
            cnt += 1;
        }
        if !words.contains_key(&pair[1]) {
            words.insert(pair[1].clone(), cnt);
            cnt += 1;
        }
    }
    let mut us = UnionSet::new(words.len());
    for pair in similar_pairs {
        us.union(words[&pair[0]], words[&pair[1]]);
    }
    for (a, b) in sentence1.into_iter().zip(sentence2) {
        if a == b { continue; }
        match (words.get(&a), words.get(&b)) {
            (Some(&i), Some(&j)) => {
                if us.find(i) != us.find(j) { return false; }
            }
            _ => return false
        }
    }
    true
}

fn main() {
    fn test(func: fn(sentence1: Vec<String>, sentence2: Vec<String>, similar_pairs: Vec<Vec<String>>) -> bool) {
        assert_eq!(func(svec!["great","acting","skills"], svec!["fine","drama","talent"], vec![svec!["great","good"], svec!["fine","good"], svec!["drama","acting"], svec!["skills","talent"]]), true);
        assert_eq!(func(svec!["I","love","leetcode"], svec!["I","love","onepiece"], vec![svec!["manga","onepiece"], svec!["platform","anime"], svec!["leetcode","platform"], svec!["anime","manga"]]), true);
        assert_eq!(func(svec!["I","love","leetcode"], svec!["I","love","onepiece"], vec![svec!["manga","hunterXhunter"], svec!["platform","anime"], svec!["leetcode","platform"], svec!["anime","manga"]]), false);
    }
    test(are_sentences_similar_two);
}
