//! 回环句

pub fn is_circular_sentence(sentence: String) -> bool {
    let v = sentence.split(' ').collect::<Vec<&str>>();
    for words in v.windows(2) {
        if *words[0].as_bytes().last().unwrap() != words[1].as_bytes()[0] {
            return false;
        }
    }
    return v[0].as_bytes()[0] == *v.last().unwrap().as_bytes().last().unwrap();
}

fn main() {
    fn test(func: fn(sentence: String) -> bool) {
        assert_eq!(func(String::from("leetcode exercises sound delightful")), true);
        assert_eq!(func(String::from("eetcode")), true);
        assert_eq!(func(String::from("Leetcode is cool")), false);
    }
    test(is_circular_sentence);
}
