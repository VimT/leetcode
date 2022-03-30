//! 不同的子序列 II

/// 用一个数组存储26个字母结尾的字符串数量，每往后读一个就把整个数组全加起来再加一更新对应字母结尾的数组
pub fn distinct_subseq_ii(s: String) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let s = s.as_bytes();
    let len = s.len();
    let mut end = vec![0; 26];
    let mut result = 0;
    for i in 0..len {
        let idx = (s[i] - b'a') as usize;
        let pre = end[idx];
        end[idx] = result + 1;
        result = (result + end[idx] - pre + MOD) % MOD;
    }
    result as i32
}

fn main() {
    assert_eq!(distinct_subseq_ii(String::from("abc")), 7);
    assert_eq!(distinct_subseq_ii(String::from("aba")), 6);
    assert_eq!(distinct_subseq_ii(String::from("aaa")), 3);
}
