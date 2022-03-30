//! 查找共用字符

use leetcode::svec;

pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let len = words.len();
    let mut cnt = vec![[0; 26]; len];
    for i in 0..len {
        for &ch in words[i].as_bytes() {
            cnt[i][(ch - b'a') as usize] += 1;
        }
    }
    let mut result = vec![];
    for i in 0..26 {
        let mut max_cnt = i32::MAX;
        for j in 0..len {
            max_cnt = max_cnt.min(cnt[j][i]);
        }
        for _ in 0..max_cnt {
            result.push(((i as u8 + b'a') as char).to_string())
        }
    }
    result
}

fn main() {
    assert_eq!(common_chars(svec!["bella", "label", "roller"]), svec!["e", "l", "l"]);
    assert_eq!(common_chars(svec!["cool", "lock", "cook"]), svec!["c", "o"]);
}
