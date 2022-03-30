//! 有效的字母异位词

use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() { return false; }
    let mut count1: HashMap<char, i32> = HashMap::new();
    let mut count2: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let count = count1.entry(c).or_insert(0);
        *count += 1;
    }
    for c in t.chars() {
        let count = count2.entry(c).or_insert(0);
        *count += 1;
    }

    return count1 == count2;
}


fn main() {
    assert_eq!(is_anagram(String::from("anagram"), String::from("nagaram")), true);
    assert_eq!(is_anagram(String::from("rat"), String::from("car")), false);
}
