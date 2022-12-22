//! 统计相似字符串对的数目

use leetcode::svec;

pub fn similar_pairs(words: Vec<String>) -> i32 {
    let words: Vec<i32> = words.into_iter().map(|x| {
        let mut num = 0;
        for &ch in x.as_bytes() {
            num |= 1 << (ch - b'a');
        }
        num
    }).collect();
    let len = words.len();
    let mut result = 0;
    for i in 0..len {
        for j in i + 1..len {
            if words[i] == words[j] {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(words: Vec<String>) -> i32) {
        assert_eq!(func(svec!["aba","aabb","abcd","bac","aabc"]), 2);
        assert_eq!(func(svec!["aabb","ab","ba"]), 3);
        assert_eq!(func(svec!["nba","cba","dba"]), 0);
    }
    test(similar_pairs);
}
