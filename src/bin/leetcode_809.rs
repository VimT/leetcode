//! 情感丰富的文字

use leetcode::svec;

pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
    fn get_cnt(s: &[u8]) -> Vec<(u8, i32)> {
        let mut cnt = vec![];
        let len = s.len();
        if len == 0 { return vec![]; }
        let mut last = s[0];
        let mut last_cnt = 1;
        for i in 1..len {
            if s[i] == last {
                last_cnt += 1;
            } else {
                cnt.push((last, last_cnt));
                last = s[i];
                last_cnt = 1;
            }
        }
        cnt.push((last, last_cnt));
        return cnt;
    }
    let s = s.as_bytes();
    let cnt = get_cnt(s);
    let len = cnt.len();
    words.into_iter().filter(|x| {
        let word_cnt = get_cnt(x.as_bytes());
        if word_cnt.len() != len { return false; }
        for i in 0..len {
            if word_cnt[i].0 != cnt[i].0 {
                return false;
            }
            if cnt[i].1 < 3 && cnt[i].1 != word_cnt[i].1 {
                return false;
            } else if cnt[i].1 >= 3 && cnt[i].1 < word_cnt[i].1 {
                return false;
            }
        }
        true
    }).count() as i32
}

fn main() {
    assert_eq!(expressive_words(String::from("dddiiiinnssssssoooo"), svec!["dinnssoo", "ddinso", "ddiinnso", "ddiinnssoo", "ddiinso", "dinsoo", "ddiinsso", "dinssoo", "dinso"]), 3);
    assert_eq!(expressive_words(String::from("heeellooo"), svec!["hello", "hi", "helo"]), 1);
    assert_eq!(expressive_words(String::from("zzzzzyyyyy"), svec!["zzyy", "zy", "zyy"]), 3);
}
