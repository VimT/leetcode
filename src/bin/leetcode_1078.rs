//! Bigram 分词

use leetcode::svec;

pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
    let split: Vec<&str> = text.split(' ').collect();
    let mut result = vec![];
    for win in split.windows(3) {
        if win[0] == first && win[1] == second {
            result.push(win[2].to_string());
        }
    }
    result
}

fn main() {
    assert_eq!(find_ocurrences(String::from("alice is a good girl she is a good student"), String::from("a"), String::from("good")), svec!["girl", "student"]);
    assert_eq!(find_ocurrences(String::from("we will we will rock you"), String::from("we"), String::from("will")), svec!["we", "rock"]);
}
