//! 前后拼接

use std::collections::{BTreeSet, HashMap};
use leetcode::svec;

pub fn before_and_after_puzzles(phrases: Vec<String>) -> Vec<String> {
    let mut first: HashMap<&str, Vec<usize>> = HashMap::new();
    let len = phrases.len();
    for i in 0..len {
        let first_word = phrases[i].split(' ').next().unwrap();
        first.entry(first_word).or_default().push(i);
    }

    let mut result = BTreeSet::new();
    for i in 0..len {
        let last_word = phrases[i].rsplit(' ').next().unwrap();
        if let Some(other) = first.get(last_word) {
            let word_len = last_word.len();
            for &j in other {
                if i != j {
                    result.insert(format!("{}{}", phrases[i], &phrases[j][word_len..]));
                }
            }
        }
    }
    result.into_iter().collect()
}

fn main() {
    fn test(func: fn(phrases: Vec<String>) -> Vec<String>) {
        assert_eq!(func(svec!["writing code","code rocks"]), vec!["writing code rocks"]);
        assert_eq!(func(svec!["mission statement",
                "a quick bite to eat",
               "a chip off the old block",
               "chocolate bar",
               "mission impossible",
               "a man on a mission",
               "block party",
               "eat my words",
               "bar of soap"]),
                   vec!["a chip off the old block party",
                        "a man on a mission impossible",
                        "a man on a mission statement",
                        "a quick bite to eat my words",
                        "chocolate bar of soap"]);
        assert_eq!(func(svec!["a","b","a"]), vec!["a"]);
    }
    test(before_and_after_puzzles);
}
