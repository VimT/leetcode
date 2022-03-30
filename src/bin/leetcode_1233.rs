//! 删除子文件夹

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

    fn find(&self, s: &[u8]) -> bool {
        let mut node = self;
        for &i in s {
            let idx = if i == b'/' { 26 } else { (i - b'a') as usize };
            match &node.children[idx] {
                None => { return false; }
                Some(v) => {
                    node = v;
                    if node.is_word { return true; }
                }
            }
        }
        false
    }
}

pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    let mut trie = Trie::new();
    let mut result = Vec::with_capacity(folder.len());
    for f in &folder {
        let mut s = f.as_bytes().to_vec();
        s.push(b'/');
        trie.insert(&s);
    }
    for f in folder {
        let s = f.as_bytes();
        if !trie.find(s) {
            result.push(f)
        }
    }
    result
}

pub fn remove_subfolders_sort(folder: Vec<String>) -> Vec<String> {
    let mut folder = folder.into_iter().map(|s| s + "/").collect::<Vec<_>>();
    let mut res = vec![];
    folder.sort_unstable();
    for f in folder {
        if res.is_empty() || !f.starts_with(res.last().unwrap()) {
            res.push(f);
        }
    }
    res.iter_mut().for_each(|s| {
        s.pop();
    });
    return res;
}

fn main() {
    assert_eq!(remove_subfolders_sort(svec!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]), svec!["/a", "/c/d", "/c/f"]);
    assert_eq!(remove_subfolders_sort(svec!["/a", "/a/b/c", "/a/b/d"]), svec!["/a"]);
    assert_eq!(remove_subfolders_sort(svec!["/a", "/a/b/c", "/a/b/d", "/abc"]), svec!["/a", "/abc"]);
    assert_eq!(remove_subfolders_sort(svec!["/a/b/c", "/a/b/d", "/a/b/ca"]), svec!["/a/b/c", "/a/b/d", "/a/b/ca"]);
}
