//! 字符串分组

use std::collections::{HashMap, VecDeque};

use leetcode::svec;

pub fn group_strings(words: Vec<String>) -> Vec<i32> {
    let mut set = HashMap::new();
    let words: Vec<i32> = words.into_iter().map(|x| {
        let mut num = 0;
        for &ch in x.as_bytes() {
            num |= 1 << (ch - b'a');
        }
        num
    }).collect();
    for word in &words {
        *set.entry(*word).or_insert(0i32) += 1;
    }
    let mut cnt = 0;
    let mut group_max_len = 0;
    for word in words {
        if set.contains_key(&word) {
            let mut q = VecDeque::new();
            q.push_back(word);
            let mut group_len = 0;
            while !q.is_empty() {
                let word = q.pop_front().unwrap();
                if let Some(cnt) = set.remove(&word) {
                    group_len += cnt;
                    let mut one = vec![];
                    let mut zero = vec![];
                    for i in 0..26 {
                        q.push_back(word ^ 1 << i);
                        if word >> i & 1 == 1 {
                            one.push(i);
                        } else {
                            zero.push(i);
                        }
                    }
                    for remove in one {
                        for &add in &zero {
                            q.push_back(word ^ 1 << remove ^ 1 << add);
                        }
                    }
                }
            }
            cnt += 1;
            group_max_len = group_max_len.max(group_len);
        }
    }
    vec![cnt, group_max_len]
}

fn main() {
    assert_eq!(group_strings(svec!["a", "b", "ab", "cde"]), vec![2, 3]);
    assert_eq!(group_strings(svec!["a", "ab", "abc"]), vec![1, 3]);
}
