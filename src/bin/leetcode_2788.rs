//! 按分隔符拆分字符串

pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
    let mut result = vec![];
    for word in words {
        for split in word.split(separator) {
            if !split.is_empty() {
                result.push(split.to_string());
            }
        }
    }
    result
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>, separator: char) -> Vec<String>) {
        assert_eq!(func(svec!["one.two.three","four.five","six"], '.'), vec!["one", "two", "three", "four", "five", "six"]);
        assert_eq!(func(svec!["$easy$","$problem$"], '$'), vec!["easy", "problem"]);
        assert_eq!(func(svec!["|||"], '|'), Vec::<String>::new());
    }
    test(split_words_by_separator);
}
