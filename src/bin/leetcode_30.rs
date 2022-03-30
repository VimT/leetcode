//! 串联所有单词的子串


use leetcode::svec;

const M: u64 = 1e9 as u64 + 7;
const B: u64 = 233;

// https://www.strchr.com/hash_functions
fn hash_word(s: &[u8]) -> usize {
    let mut hash: usize = 0xB16B00B5;
    let m: usize = 33;
    for i in 0..s.len() {
        hash = m ^ hash * s[i] as usize;
    }
    hash
}

/// hash法
pub fn find_substring_hash(s: String, words: Vec<String>) -> Vec<i32> {
    fn hash(s: &str) -> u64 {
        let mut ans = 0;
        for i in s.bytes() {
            ans = (ans * B + i as u64) % M;
        }
        ans
    }
    if words.len() == 0 { return vec![]; }
    let word_len = words[0].len();
    let all_len = word_len * words.len();
    if all_len > s.len() { return vec![]; }
    let set: std::collections::HashSet<&str> = words.iter().map(|x| x.as_str()).collect();
    let wh: u64 = words.iter().map(|x| hash(x)).sum();
    let mut ans = vec![];
    for i in 0..=s.len() - all_len {
        let ns = &s[i..i + all_len];
        let mut th = 0;
        for j in (0..all_len).step_by(word_len) {
            let w = &ns[j..j + word_len];
            if !set.contains(w) {
                break;
            }
            th += hash(w);
        }
        if th == wh {
            ans.push(i as i32);
        }
    }
    ans
}

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let mut result = vec![];
    if s.is_empty() || words.is_empty() || words.first().unwrap().is_empty() {
        return result;
    }
    let (word_len, word_cnt) = (words.first().unwrap().len(), words.len());
    if word_len * word_cnt > s.len() { return result; }
    let mut needs = std::collections::BTreeMap::new();
    words.iter().for_each(|s| {
        *needs.entry(s.as_str()).or_insert(0) += 1;
    });
    for j in 0..word_len {
        let mut current = std::collections::BTreeMap::new();
        let mut n = 0;
        let mut i = j;
        while i <= s.len() - word_cnt * word_len {
            let mut removed = false;
            while n < word_cnt {
                let s2 = &s[i + n * word_len..i + (n + 1) * word_len];
                if needs.contains_key(s2) {
                    let entry = current.entry(s2).or_insert(0);
                    *entry += 1;
                    // Case 3, word is correct, but overflow
                    if *current.get(s2).unwrap() > *needs.get(s2).unwrap() {
                        removed = true;
                        let mut remove_num = 0;
                        while *current.get(s2).unwrap() > *needs.get(s2).unwrap() {
                            let first_word = &s[i + remove_num * word_len..i + (remove_num + 1) * word_len];
                            let entry = current.entry(first_word).or_insert(0);
                            *entry -= 1;
                            remove_num += 1;
                        }
                        n -= remove_num - 1;
                        i += (remove_num - 1) * word_len;
                        break;
                    }
                } else {
                    // Case 2, word is not correct
                    current.clear();
                    i += n * word_len;
                    n = 0;
                    break;
                }
                n += 1;
            }
            if n == word_cnt {
                result.push(i as i32);
            }
            // Case 1, completely match
            if n > 0 && !removed {
                let first_word = &s[i..i + word_len];
                let entry = current.entry(first_word).or_insert(0);
                *entry -= 1;
                n -= 1;
            }
            i += word_len;
        }
    }
    return result;
}

fn main() {
    fn test(func: fn(s: String, words: Vec<String>) -> Vec<i32>) {
        assert_eq!(func(String::from("barfoothefoobarman"), svec!["foo", "bar"]), vec![0, 9]);
        assert_eq!(func(String::from("wordgoodgoodgoodbestword"), svec!["word", "good", "best", "word"]), vec![]);
        assert_eq!(func(String::from("barfoofoobarthefoobarman"), svec!["bar", "foo", "the"]), vec![6, 9, 12]);
    }
    test(find_substring);
    test(find_substring_hash);
}
