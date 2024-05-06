//! 最长公共后缀查询

pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
    #[derive(Default)]
    struct Trie {
        children: [Option<Box<Trie>>; 26],
        word: usize,
        word_len: usize,
    }
    impl Trie {
        fn insert(&mut self, word: &[u8], word_index: usize) {
            let mut node = self;
            for &ch in word.iter().rev().chain(&[0]) {
                if (node.word_len > 0 && word.len() < node.word_len) || (node.word_len == 0) {
                    node.word = word_index;
                    node.word_len = word.len();
                }
                if ch > 0 { node = node.children[(ch - b'a') as usize].get_or_insert_with(Box::<Trie>::default); }
            }
        }

        fn query(&self, word: &[u8]) -> usize {
            let mut node = self;
            for &ch in word.iter().rev() {
                if let Some(child) = &node.children[(ch - b'a') as usize] {
                    node = child;
                } else {
                    return node.word;
                }
            }
            node.word
        }
    }

    let mut trie = Trie::default();
    for (i, word) in words_container.iter().enumerate() {
        trie.insert(word.as_bytes(), i);
    }
    words_query.into_iter().map(|word| {
        trie.query(word.as_bytes()) as i32
    }).collect()
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32>) {
        assert_eq!(func(svec!["abcd","bcd","xbcd"], svec!["cd","bcd","xyz"]), vec![1, 1, 1]);
        assert_eq!(func(svec!["abcdefgh","poiuygh","ghghgh"], svec!["gh","acbfgh","acbfegh"]), vec![2, 0, 2]);
    }
    test(string_indices);
}
