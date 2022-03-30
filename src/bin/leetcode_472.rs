//! 连接词

use leetcode::svec;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_word: bool,
}

impl Trie {
    fn new() -> Box<Self> {
        Box::new(Default::default())
    }

    fn insert(&mut self, word: &[u8]) {
        let mut node = self;
        for &char in word {
            let nxt = node.children[(char - b'a') as usize].get_or_insert(Trie::new());
            node = nxt;
        }
        node.is_word = true;
    }
}

pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
    fn dfs(root: &Trie, word: &[u8], is_first: bool) -> bool {
        let mut node = root;
        for i in 0..word.len() {
            if let Some(nxt) = &node.children[(word[i] - b'a') as usize] {
                node = nxt;
                if nxt.is_word {
                    if i == word.len() - 1 {
                        return !is_first;
                    }
                    if dfs(root, &word[i + 1..], false) {
                        return true;
                    }
                }
            } else {
                break;
            }
        }
        false
    }
    words.sort_unstable_by_key(|x| x.len());
    let mut trie = Trie::new();
    let mut result = Vec::with_capacity(words.len());
    for i in 0..words.len() {
        if words[i].len() == 0 { continue; }
        if dfs(&trie, words[i].as_bytes(), true) {
            result.push(words[i].clone());
        } else {
            trie.insert(words[i].as_bytes());
        }
    }
    result
}

fn main() {
    assert_eq!(find_all_concatenated_words_in_a_dict(svec!["cat", "cats", "catsdogcats", "dog", "dogcatsdog", "hippopotamuses", "rat", "ratcatdogcat"]), svec!["dogcatsdog", "catsdogcats", "ratcatdogcat"]);
    assert_eq!(find_all_concatenated_words_in_a_dict(svec![""]), svec![]);
    assert_eq!(find_all_concatenated_words_in_a_dict(svec!["cat", "dog", "catdog"]), svec!["catdog"]);
}
