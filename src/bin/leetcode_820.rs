//! 单词的压缩编码

use std::collections::HashSet;

use leetcode::svec;

pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
    let mut set = HashSet::new();
    words.sort_unstable_by_key(|x| x.len());
    let mut result = 0;
    for word in words.iter().rev() {
        let w = word.as_bytes();
        if set.contains(w) {
            continue;
        }
        for i in 0..w.len() {
            set.insert(&w[i..]);
        }
        result += w.len() as i32 + 1;
    }
    result
}


#[derive(Default, Debug)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, s: &[u8]) {
        let mut node = self;
        for &i in s {
            let idx = (i - b'a') as usize;
            node = node.children[idx].get_or_insert_with(Box::<Trie>::default);
        }
        node.is_word = true;
    }
}

pub fn minimum_length_encoding_trie(words: Vec<String>) -> i32 {
    let mut trie = Trie::new();
    for word in words {
        let mut w = word.as_bytes().to_vec();
        w.reverse();
        trie.insert(&w);
    }

    fn total_len(node: &Trie, cur: i32) -> i32 {
        let mut result = 0;
        for i in 0..26 {
            if let Some(sub) = &node.children[i] {
                result += total_len(&sub, cur + 1);
            }
        }
        if result > 0 { result } else { cur }
    }
    total_len(&trie, 1)
}

fn main() {
    assert_eq!(minimum_length_encoding_trie(svec!["time", "me", "bell"]), 10);
    assert_eq!(minimum_length_encoding_trie(svec!["t"]), 2);
}

