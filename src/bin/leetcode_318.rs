//! 最大单词长度乘积

use leetcode::svec;

pub fn max_product(words: Vec<String>) -> i32 {
    let len = words.len();
    let mut bit_words = vec![0_u32; len];
    for i in 0..len {
        let mut num = 0;
        for char in words[i].as_bytes() {
            num |= 1 << (*char - b'a');
        }
        bit_words[i] = num;
    }
    let mut result = 0;
    for i in 0..len {
        for j in i + 1..len {
            if bit_words[i] & bit_words[j] == 0 {
                result = result.max(words[i].len() * words[j].len());
            }
        }
    }
    result as i32
}

fn main() {
    assert_eq!(max_product(svec!["abcw","baz","foo","bar","xtfn","abcdef"]), 16);
    assert_eq!(max_product(svec!["a","ab","abc","d","cd","bcd","abcd"]), 4);
    assert_eq!(max_product(svec!["a","aa","aaa","aaaa"]), 0);
}
