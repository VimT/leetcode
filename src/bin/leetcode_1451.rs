//! 重新排列句子中的单词

pub fn arrange_words(mut text: String) -> String {
    text[..1].make_ascii_lowercase();
    let mut s: Vec<&str> = text.split(" ").collect();
    s.sort_by_key(|x| x.len() as i32);
    let mut result = s.join(" ");
    result[..1].make_ascii_uppercase();
    result
}

fn main() {
    fn test(func: fn(text: String) -> String) {
        assert_eq!(func(String::from("Leetcode is cool")), String::from("Is cool leetcode"));
        assert_eq!(func(String::from("Keep calm and code on")), String::from("On and keep calm code"));
        assert_eq!(func(String::from("To be or not to be")), String::from("To be or to be not"));
    }
    test(arrange_words);
}
