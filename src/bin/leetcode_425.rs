//! 单词方块

use std::collections::HashMap;
use leetcode::{svec, unorder};

pub fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    let n = words[0].len();
    for word in &words {
        for prefix_len in 1..n {
            map.entry(word[..prefix_len].as_bytes()).or_insert(vec![]).push(word.as_bytes());
        }
    }
    let mut result = vec![];
    let mut cur = vec![&b""[..]; n];
    for first in &words {
        let first = first.as_bytes();
        cur[0] = first;
        if n > 1 {
            if let Some(words) = map.get(&[first[1]][..]) {
                for second in words {
                    cur[1] = second;
                    if n > 2 {
                        if let Some(words) = map.get(&[first[2], second[2]][..]) {
                            for third in words {
                                cur[2] = third;
                                if n > 3 {
                                    if let Some(words) = map.get(&[first[3], second[3], third[3]][..]) {
                                        for fourth in words {
                                            cur[3] = fourth;
                                            result.push(cur.iter().map(|x| unsafe { String::from_utf8_unchecked(x.to_vec()) }).collect());
                                        }
                                    }
                                } else {
                                    result.push(cur.iter().map(|x| unsafe { String::from_utf8_unchecked(x.to_vec()) }).collect());
                                }
                            }
                        }
                    } else {
                        result.push(cur.iter().map(|x| unsafe { String::from_utf8_unchecked(x.to_vec()) }).collect());
                    }
                }
            }
        } else {
            result.push(cur.iter().map(|x| unsafe { String::from_utf8_unchecked(x.to_vec()) }).collect());
        }
    }
    result
}

/// 回溯
pub fn word_squares2(words: Vec<String>) -> Vec<Vec<String>> {
    fn backtrace(step: i32, n: i32, map: &HashMap<&[u8], Vec<&[u8]>>, cur: &mut Vec<Vec<u8>>, result: &mut Vec<Vec<String>>) {
        if step == n {
            result.push(cur.iter().map(|x| unsafe { String::from_utf8_unchecked(x.to_vec()) }).collect());
            return;
        }
        let prefix: Vec<u8> = cur.iter().map(|x| x[step as usize]).collect();
        if let Some(candidates) = map.get(&prefix[..]) {
            for candidate in candidates {
                cur.push(candidate.to_vec());
                backtrace(step + 1, n, map, cur, result);
                cur.pop();
            }
        }
    }
    let mut map = HashMap::new();
    let n = words[0].len();
    for word in &words {
        for prefix_len in 1..n {
            map.entry(word[..prefix_len].as_bytes()).or_insert(vec![]).push(word.as_bytes());
        }
    }
    let mut result = vec![];
    for word in &words {
        let mut cur = vec![word.as_bytes().to_vec()];
        backtrace(1, n as i32, &map, &mut cur, &mut result);
    }
    result
}

fn main() {
    fn test(func: fn(words: Vec<String>) -> Vec<Vec<String>>) {
        assert_eq!(unorder(func(svec!["a"])), vec![svec!["a"]]);
        assert_eq!(unorder(func(svec!["area","lead","wall","lady","ball"])), vec![svec!["ball","area","lead","lady"], svec!["wall","area","lead","lady"]]);
        assert_eq!(unorder(func(svec!["abat","baba","atan","atal"])), vec![svec!["baba","abat","baba","atal"], svec!["baba","abat","baba","atan"]]);
    }
    test(word_squares);
    test(word_squares2);
}
