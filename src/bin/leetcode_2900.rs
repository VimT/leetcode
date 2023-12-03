//! 最长相邻不相等子序列 I

pub fn get_words_in_longest_subsequence(_: i32, words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    let mut result = vec![];
    let mut last = -1;
    for (word, group) in words.into_iter().zip(groups) {
        if group != last {
            result.push(word);
        }
        last = group;
    }
    result
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(n: i32, words: Vec<String>, groups: Vec<i32>) -> Vec<String>) {
        assert_eq!(func(3, svec!["e","a","b"], vec![0, 0, 1]), vec!["e", "b"]);
        assert_eq!(func(4, svec!["a","b","c","d"], vec![1, 0, 1, 1]), vec!["a", "b", "c"]);
    }
    test(get_words_in_longest_subsequence);
}
