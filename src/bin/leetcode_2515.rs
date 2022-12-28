//! 到目标字符串的最短距离

use leetcode::svec;

pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
    let len = words.len();
    let mut result = i32::MAX;
    for (i, word) in words.into_iter().enumerate() {
        if word == target {
            result = result.min((i as i32 - start_index).abs()).min(len as i32 - (i as i32 - start_index).abs());
        }
    }
    if result == i32::MAX { -1 } else { result }
}

fn main() {
    fn test(func: fn(words: Vec<String>, target: String, start_index: i32) -> i32) {
        assert_eq!(func(svec!["hello","i","am","leetcode","hello"], String::from("hello"), 1), 1);
        assert_eq!(func(svec!["a","b","leetcode"], String::from("leetcode"), 0), 1);
        assert_eq!(func(svec!["i","eat","leetcode"], String::from("ate"), 0), -1);
    }
    test(closet_target);
}
