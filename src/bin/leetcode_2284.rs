//! 最多单词数的发件人

use std::collections::HashMap;
use leetcode::svec;

pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
    let mut cnt: HashMap<String, usize> = HashMap::new();
    for (message, sender) in messages.into_iter().zip(senders.into_iter()) {
        *cnt.entry(sender).or_default() += message.split(' ').count();
    }
    cnt.into_iter().max_by(|a, b| (a.1, &a.0).cmp(&(b.1, &b.0))).unwrap().0
}

fn main() {
    fn test(func: fn(messages: Vec<String>, senders: Vec<String>) -> String) {
        assert_eq!(func(svec!["Hello userTwooo","Hi userThree","Wonderful day Alice","Nice day userThree"], svec!["Alice","userTwo","userThree","Alice"]), String::from("Alice"));
        assert_eq!(func(svec!["How is leetcode for everyone","Leetcode is useful for practice"], svec!["Bob","Charlie"]), String::from("Charlie"));
    }
    test(largest_word_count);
}
