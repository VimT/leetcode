//! 单词缩写

use std::collections::HashMap;
use leetcode::svec;

// 贪心，对于每个单词，先缩写，再看是不是会有重复
pub fn words_abbreviation(words: Vec<String>) -> Vec<String> {
    let len = words.len();
    fn abbrev(word: &[u8], idx: usize) -> String {
        if word.len() <= idx + 3 { unsafe { return String::from_utf8_unchecked(word.to_vec()); } }
        let mut result = vec![];
        result.extend_from_slice(&word[..=idx]);
        result.extend_from_slice((word.len() - idx - 2).to_string().as_bytes());
        result.push(word[word.len() - 1]);
        unsafe { String::from_utf8_unchecked(result) }
    }
    let mut result: Vec<String> = words.iter().map(|x| abbrev(x.as_bytes(), 0)).collect();
    let mut prefix = vec![0; len];
    for i in 0..len {
        loop {
            let mut dupes = vec![i];
            for j in i + 1..len {
                if result[i] == result[j] {
                    dupes.push(j);
                }
            }
            if dupes.len() == 1 { break; }
            for j in dupes {
                prefix[j] += 1;
                result[j] = abbrev(words[j].as_bytes(), prefix[j]);
            }
        }
    }
    result
}

/// 分组+最短公共前缀。 group的单词排序，计算lcp。或者对group的单词trie树一下
pub fn words_abbreviation_group(words: Vec<String>) -> Vec<String> {
    fn lcp(word1: &[u8], word2: &[u8]) -> usize {
        let mut i = 0;
        while i < word1.len() && i < word2.len() && word1[i] == word2[i] {
            i += 1;
        }
        i
    }

    let mut groups = HashMap::new();
    for (idx, word) in words.iter().enumerate() {
        let s = word.as_bytes();
        groups.entry((s.len(), s[0], *s.last().unwrap())).or_insert(vec![]).push((word, idx));
    }
    let mut result = vec![String::new(); words.len()];
    for ((len, _, last), mut words) in groups {
        words.sort_unstable();
        let mut lcp_list = vec![0; words.len()];
        for (i, (word, _)) in words.iter().enumerate() {
            if i > 0 {
                let word2 = words[i - 1].0.as_bytes();
                lcp_list[i] = lcp(word.as_bytes(), word2);
                lcp_list[i - 1] = lcp_list[i - 1].max(lcp_list[i]);
            }
        }

        for ((word, idx), p) in words.iter().zip(lcp_list) {
            if len <= p + 3 {
                result[*idx] = word.to_string();
            } else {
                result[*idx] = word[..=p].to_string() + &(len - p - 2).to_string() + &(last as char).to_string();
            }
        }
    }
    result
}


fn main() {
    fn test(func: fn(words: Vec<String>) -> Vec<String>) {
        assert_eq!(func(svec!["like","god","internal","me","internet","interval","intension","face","intrusion"]), vec!["l2e", "god", "internal", "me", "i6t", "interval", "inte4n", "f2e", "intr4n"]);
        assert_eq!(func(svec!["aa","aaa"]), vec!["aa", "aaa"]);
    }
    test(words_abbreviation);
    test(words_abbreviation_group);
}
