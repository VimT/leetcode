//! 搜索推荐系统

use leetcode::svec;


#[derive(Default, Debug)]
pub struct Trie<'a> {
    children: [Option<Box<Trie<'a>>>; 26],
    word_list: Vec<&'a str>,
}

impl<'a> Trie<'a> {
    pub fn insert(&mut self, s: &'a str) {
        let mut node = self;
        for i in s.as_bytes().iter() {
            let idx = (i - b'a') as usize;
            node = node.children[idx].get_or_insert_with(Box::<Trie>::default);
            node.word_list.push(s);
        }
    }

}

pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    products.sort_unstable();
    let mut trie = Trie::default();
    for pro in &products {
        trie.insert(pro);
    }
    let mut result = vec![vec![]; search_word.len()];
    let mut node = &trie;
    for (i, &ch) in search_word.as_bytes().iter().enumerate() {
        if let Some(child) = &node.children[(ch - b'a') as usize] {
            node = child;
            result[i] = node.word_list.iter().take(3).map(|x| x.to_string()).collect();
        } else { break; }
    }
    result
}

fn main() {
    fn test(func: fn(products: Vec<String>, search_word: String) -> Vec<Vec<String>>) {
        assert_eq!(func(svec!["mobile","mouse","moneypot","monitor","mousepad"], String::from("mouse")), vec![svec!["mobile","moneypot","monitor"], svec!["mobile","moneypot","monitor"], svec!["mouse","mousepad"], svec!["mouse","mousepad"], svec!["mouse","mousepad"]]);
        assert_eq!(func(svec!["havana"], String::from("havana")), vec![svec!["havana"], svec!["havana"], svec!["havana"], svec!["havana"], svec!["havana"], svec!["havana"]]);
    }
    test(suggested_products);
}
