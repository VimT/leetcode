//! 字符流

use leetcode::svec;

#[derive(Default, Debug)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_word: bool,
}

impl Trie {
    pub fn insert(&mut self, s: &String) {
        let mut node = self;
        for i in s.as_bytes().iter().rev() {
            let idx = (i - b'a') as usize;
            node = node.children[idx].get_or_insert_with(Box::<Trie>::default);
        }
        node.is_word = true;
    }

    pub fn search(&self, s: &[u8]) -> bool {
        let mut node = self;
        for i in s.iter().rev() {
            let idx = (i - b'a') as usize;
            if let Some(child) = &node.children[idx] {
                node = child;
                if child.is_word { return true; }
            } else { break; }
        }
        false
    }
}

struct StreamChecker {
    trie: Trie,
    stream: Vec<u8>,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::default();
        for word in words {
            trie.insert(&word);
        }
        Self { trie, stream: vec![] }
    }

    fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter as u8);
        self.trie.search(&self.stream)
    }
}

fn main() {
    let mut sc = StreamChecker::new(svec!["cd", "f", "kl"]);
    assert_eq!(sc.query('a'), false); // 返回 false
    assert_eq!(sc.query('b'), false); // 返回 false
    assert_eq!(sc.query('c'), false); // 返回 false
    assert_eq!(sc.query('d'), true); // 返回 true ，因为 'cd' 在 words 中
    assert_eq!(sc.query('e'), false); // 返回 false
    assert_eq!(sc.query('f'), true); // 返回 true ，因为 'f' 在 words 中
    assert_eq!(sc.query('g'), false); // 返回 false
    assert_eq!(sc.query('h'), false); // 返回 false
    assert_eq!(sc.query('i'), false); // 返回 false
    assert_eq!(sc.query('j'), false); // 返回 false
    assert_eq!(sc.query('k'), false); // 返回 false
    assert_eq!(sc.query('l'), true); // 返回 true ，因为 'kl' 在 words 中
}