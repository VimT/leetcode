//! 添加与搜索单词 - 数据结构设计


#[derive(Default)]
struct TrieNode {
    sub: Box<[Option<TrieNode>; 26]>,
    end: bool,
}

struct WordDictionary {
    root: TrieNode,
}


impl WordDictionary {
    fn new() -> Self {
        WordDictionary { root: TrieNode::default() }
    }

    fn add_word(&mut self, word: String) {
        let word = word.as_bytes();
        let mut node = &mut self.root;
        for s in word {
            let idx = (s - b'a') as usize;
            if let Option::None = node.sub[idx] {
                node.sub[(s - b'a') as usize] = Some(TrieNode::default())
            }
            node = node.sub[idx].as_mut().unwrap();
        }
        node.end = true;
    }

    fn do_search(&self, node: &TrieNode, word: &[u8], start: usize) -> bool {
        if start == word.len() { return node.end; }
        let s = word[start];
        return if s == b'.' {
            for i in 0..26_usize {
                if let Some(nn) = &node.sub[i] {
                    if self.do_search(nn, word, start + 1) {
                        return true;
                    }
                }
            }
            false
        } else {
            let idx = (s - b'a') as usize;
            if let Option::None = node.sub[idx] {
                return false;
            }
            self.do_search(node.sub[idx].as_ref().unwrap(), word, start + 1)
        };
    }

    fn search(&self, word: String) -> bool {
        let node = &self.root;
        self.do_search(node, word.as_bytes(), 0)
    }
}

fn main() {
    let mut wd = WordDictionary::new();
    wd.add_word(String::from("bad"));
    wd.add_word(String::from("dad"));
    wd.add_word(String::from("mad"));
    assert_eq!(wd.search(String::from("pad")), false); // return False
    assert_eq!(wd.search(String::from("bad")), true); // return True
    assert_eq!(wd.search(String::from(".ad")), true); // return True
    assert_eq!(wd.search(String::from("b..")), true); // return True

    let mut wd2 = WordDictionary::new();
    wd2.add_word(String::from("a"));
    wd2.add_word(String::from("a"));
    assert_eq!(wd2.search(String::from(".")), true);
    assert_eq!(wd2.search(String::from("a")), true);
    assert_eq!(wd2.search(String::from("aa")), false);
    assert_eq!(wd2.search(String::from("a")), true);
    assert_eq!(wd2.search(String::from(".a")), false);
    assert_eq!(wd2.search(String::from("a.")), false);

    let mut wd3 = WordDictionary::new();
    wd3.add_word(String::from("bad"));
    wd3.add_word(String::from("dad"));
    wd3.add_word(String::from("mad"));
    assert_eq!(wd3.search(String::from("pad")), false);
    assert_eq!(wd3.search(String::from("bad")), true);
    assert_eq!(wd3.search(String::from(".ad")), true);
    assert_eq!(wd3.search(String::from("b..")), true);
}
