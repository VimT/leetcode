//! 字符串的前缀分数和

use leetcode::svec;

#[derive(Default, Debug)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    prefix_num: i32,
}

impl Trie {
    pub fn insert(&mut self, s: &[u8]) {
        let mut node = self;
        for &i in s.iter() {
            let idx = (i - b'a') as usize;
            node = node.children[idx].get_or_insert_with(Box::<Trie>::default);
            node.prefix_num += 1;
        }
    }

    pub fn get_score(&self, s: &[u8]) -> i32 {
        let mut node = self;
        let mut sum = 0;
        for &i in s.iter() {
            let idx = (i - b'a') as usize;
            if let Some(child) = &node.children[idx] {
                node = child;
                sum += node.prefix_num;
            } else { break; }
        }
        sum
    }
}


pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    let mut trie = Trie::default();
    for word in &words {
        trie.insert(word.as_bytes());
    }
    words.into_iter().map(|x| {
        trie.get_score(x.as_bytes())
    }).collect()
}


fn main() {
    fn test(func: fn(words: Vec<String>) -> Vec<i32>) {
        assert_eq!(func(svec!["abc","ab","bc","b"]), vec![5, 4, 3, 2]);
        assert_eq!(func(svec!["abcd"]), vec![4]);
    }
    test(sum_prefix_scores);
}
