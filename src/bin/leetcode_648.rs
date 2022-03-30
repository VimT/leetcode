//! 单词替换

use leetcode::svec;

#[derive(Default, Debug)]
pub struct Trie {
    children: [Option<Box<Trie>>; 27],
    is_word: bool,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, s: &[u8]) {
        let mut node = self;
        for &i in s {
            let idx = if i == b'/' { 26 } else { (i - b'a') as usize };
            node = node.children[idx].get_or_insert_with(Box::<Trie>::default);
        }
        node.is_word = true;
    }

    fn find(&self, s: &[u8]) -> usize {
        let mut node = self;
        for i in 0..s.len() {
            let ch = s[i];
            let idx = if ch == b'/' { 26 } else { (ch - b'a') as usize };
            match &node.children[idx] {
                None => { return s.len(); }
                Some(v) => {
                    node = v;
                    if node.is_word { return i + 1; }
                }
            }
        }
        s.len()
    }
}


pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let mut trie = Trie::new();
    for word in dictionary {
        let w = word.as_bytes();
        trie.insert(w);
    }
    sentence.split(' ').map(|x| {
        let s = x.as_bytes();
        &x[..trie.find(s)]
    }).collect::<Vec<&str>>().join(" ")
}

fn main() {
    assert_eq!(replace_words(svec!["cat", "bat", "rat"], String::from("the cattle was rattled by the battery")), String::from("the cat was rat by the bat"));
    assert_eq!(replace_words(svec!["a", "b", "c"], String::from("aadsfasf absbs bbab cadsfafs")), String::from("a a b c"));
}
