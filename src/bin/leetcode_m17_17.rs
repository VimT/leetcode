//! 面试题 17.17. 多次搜索


struct Trie {
    children: [Option<Box<Trie>>; 26],
    val: i32,
}

impl Trie {
    fn new() -> Self {
        Trie { children: Default::default(), val: -1 }
    }

    fn insert(&mut self, word: &[u8], val: i32) {
        let mut node = self;
        for w in word {
            let idx = (*w - b'a') as usize;
            if node.children[idx].is_none() {
                node.children[idx] = Some(Box::new(Trie::new()));
            }
            node = node.children[idx].as_mut().unwrap();
        }
        node.val = val;
    }

    fn search(&self, word: &[u8]) -> Vec<i32> {
        let mut node = self;
        let mut result = vec![];
        for &w in word {
            let idx = (w - b'a') as usize;
            if let Some(next) = node.children[idx].as_ref() {
                if next.val != -1 {
                    result.push(next.val);
                }
                node = next;
            } else {
                break;
            }
        }
        result
    }
}

pub fn multi_search(big: String, smalls: Vec<String>) -> Vec<Vec<i32>> {
    let mut trie = Trie::new();
    for i in 0..smalls.len() {
        trie.insert(smalls[i].as_bytes(), i as i32);
    }
    let mut result = vec![vec![]; smalls.len()];
    let s = big.as_bytes();
    for i in 0..s.len() {
        for j in trie.search(&s[i..]) {
            result[j as usize].push(i as i32);
        }
    }
    result
}

fn main() {
    assert_eq!(multi_search("mississippi".into(), ["is", "ppi", "hi", "sis", "i", "ssippi"].iter().map(|x| x.to_string()).collect()),
               vec![vec![1, 4], vec![8], vec![], vec![3], vec![1, 4, 7, 10], vec![5]])
}
