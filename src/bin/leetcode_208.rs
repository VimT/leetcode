//! 实现 Trie (前缀树)
#[derive(Default)]
pub struct Trie {
    root: Node,
}

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }


    fn get_node(&self, s: &str) -> Option<&Node> {
        let mut node = &self.root;
        for c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            match &node.children[idx] {
                Some(v) => node = &v,
                None => return None,
            }
        }
        Some(node)
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.as_bytes() {
            let idx = (c - b'a') as usize;
            let next = &mut node.children[idx];
            node = next.get_or_insert_with(Box::<Node>::default);
        }
        node.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        return self.get_node(&word).map_or(false, |v| v.is_word);
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        return self.get_node(&prefix).is_some();
    }
}

fn main() {
    let mut obj = Trie::new();
    let word = "apple".to_string();
    obj.insert(word);
    assert_eq!(obj.starts_with("app".to_string()), true);
    assert_eq!(obj.search("apple".to_string()), true);
}

