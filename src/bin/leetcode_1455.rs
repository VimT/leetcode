//! 检查单词是否为句中其他单词的前缀

pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    for (word, i) in sentence.split(" ").zip(1..) {
        if word.starts_with(&search_word) {
            return i;
        }
    }
    -1
}

fn main() {
    fn test(func: fn(sentence: String, search_word: String) -> i32) {
        assert_eq!(func(String::from("i love eating burger"), String::from("burg")), 4);
        assert_eq!(func(String::from("this problem is an easy problem"), String::from("pro")), 2);
        assert_eq!(func(String::from("i am tired"), String::from("you")), -1);
    }
    test(is_prefix_of_word);
}
