//! 前缀和后缀搜索

use leetcode::svec;

#[derive(Default, Debug)]
pub struct Trie {
    children: [Option<Box<Trie>>; 27],
    idx: usize,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert_with_suffix(&mut self, word: &[u8], suffix: &[u8], idx: usize) {
        let mut node = self;
        for &ch in suffix {
            node = node.children[(ch - b'a') as usize].get_or_insert_with(Box::<Trie>::default);
        }
        node = node.children[26].get_or_insert_with(Box::<Trie>::default);
        for &ch in word {
            node = node.children[(ch - b'a') as usize].get_or_insert_with(Box::<Trie>::default);
            node.idx = idx;
        }
    }

    /// apple 分解成 e#apple, le#apple, ple#apple, pple#apple, apple#apple 插入trie树
    fn insert(&mut self, word: &[u8], idx: usize) {
        let len = word.len();
        for i in (0..len).rev() {
            self.insert_with_suffix(word, &word[i..], idx);
        }
    }

    fn find(&self, prefix: &[u8], suffix: &[u8]) -> Option<usize> {
        let mut node = self;
        for &ch in suffix {
            match &node.children[(ch - b'a') as usize] {
                None => { return None; }
                Some(v) => { node = v; }
            }
        }
        node = match node.children[26].as_ref() {
            None => return None,
            Some(v) => v,
        };
        for &ch in prefix {
            match &node.children[(ch - b'a') as usize] {
                None => { return None; }
                Some(v) => { node = v; }
            }
        }
        Some(node.idx)
    }
}


struct WordFilter {
    trie: Trie,
}


impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for (idx, word) in words.into_iter().enumerate() {
            trie.insert(word.as_bytes(), idx);
        }
        Self { trie }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        self.trie.find(prefix.as_bytes(), suffix.as_bytes()).map(|x| x as i32).unwrap_or(-1)
    }
}

fn main() {
    let wf = WordFilter::new(svec!["apple"]);
    assert_eq!(wf.f(String::from("a"), String::from("e")), 0); // 返回 0 ，因为下标为 0 的单词的 prefix = "a" 且 suffix = 'e" 。
    assert_eq!(wf.f(String::from("aaa"), String::from("e")), -1);
}