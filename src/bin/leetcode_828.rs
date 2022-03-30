//! 统计子串中的唯一字符

/// s 中有多少个子串只包含一个 "A"
/// 计算从当前位置到下一个字符出现的位置之间的间隔，就是只包含一个 char 的多少个字串
pub fn unique_letter_string(s: String) -> i32 {
    const MOD: usize = 1e9 as usize + 7;
    let s = s.as_bytes();
    let mut idx = vec![vec![]; 26];
    let mut peek = vec![0; 26];
    let len = s.len();
    for i in 0..len {
        idx[(s[i] - b'A') as usize].push(i);
    }
    #[inline]
    fn get(idx: &Vec<Vec<usize>>, peek: &Vec<usize>, c: usize) -> usize {
        idx[c][peek[c] + 1] - idx[c][peek[c]]
    }
    let mut cur: usize = 0;
    for i in 0..26 {
        if !idx[i].is_empty() {
            idx[i].push(len);
            idx[i].push(len);
            cur += get(&idx, &peek, i);
        }
    }
    let mut result = 0;
    for &ch in s {
        result = (result + cur) % MOD;
        let ch_idx = (ch - b'A') as usize;
        let old = get(&idx, &peek, ch_idx);
        peek[ch_idx] += 1;
        cur = (cur + get(&idx, &peek, ch_idx) - old) % MOD;
    }
    result as i32
}

fn main() {
    assert_eq!(unique_letter_string(String::from("ABC")), 10);
    assert_eq!(unique_letter_string(String::from("ABA")), 8);
    assert_eq!(unique_letter_string(String::from("LEETCODE")), 92);
}
