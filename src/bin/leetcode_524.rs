//! 通过删除字母匹配到字典里最长单词

use leetcode::svec;

pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
    let s = s.as_bytes();
    let mut result = String::new();
    for word in dictionary {
        let w = word.as_bytes();
        let mut i = 0;
        for j in 0..s.len() {
            if s[j] == w[i] {
                i += 1;
                if i == w.len() { break; }
            }
        }
        if i == w.len() {
            if w.len() > result.len() {
                result = word;
            } else if w.len() == result.len() && word < result {
                result = word;
            }
        }
    }
    result
}


pub fn find_longest_word_optimise(s: String, mut dictionary: Vec<String>) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![[0; 26]; len];
    dp.push([len; 26]);
    for i in (0..len).rev() {
        for j in 0..26 {
            dp[i][j] = if s[i] - b'a' == j as u8 { i } else { dp[i + 1][j] };
        }
    }
    dictionary.sort_unstable_by(|x, y| if x.len() == y.len() { x.cmp(y) } else { y.len().cmp(&x.len()) });
    for word in dictionary {
        let w = word.as_bytes();
        let (mut i, mut j) = (0, 0);
        while i < w.len() && j < s.len() {
            if dp[j][(w[i] - b'a') as usize] >= len {
                break;
            }
            j = dp[j][(w[i] - b'a') as usize] + 1;
            i += 1;
        }
        if i == w.len() {
            return word;
        }
    }
    String::new()
}

fn main() {
    assert_eq!(find_longest_word_optimise(String::from("abpcplea"), svec!["ale", "apple", "monkey", "plea"]), String::from("apple"));
    assert_eq!(find_longest_word(String::from("abpcplea"), svec!["ale", "apple", "monkey", "plea"]), String::from("apple"));
    assert_eq!(find_longest_word_optimise(String::from("abpcplea"), svec!["a", "b", "c"]), String::from("a"));
    assert_eq!(find_longest_word(String::from("abpcplea"), svec!["a", "b", "c"]), String::from("a"));
}
