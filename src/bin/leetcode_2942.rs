//! 查找包含给定字符的单词

pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    words.into_iter().enumerate().filter_map(|(i, word)| {
        word.find(x).map(|_| i as i32)
    }).collect()
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>, x: char) -> Vec<i32>) {
        assert_eq!(func(svec!["leet","code"], 'e'), vec![0, 1]);
        assert_eq!(func(svec!["abc","bcd","aaaa","cbc"], 'a'), vec![0, 2]);
        assert_eq!(func(svec!["abc","bcd","aaaa","cbc"], 'z'), vec![]);
    }
    test(find_words_containing);
}
