//! 得到 K 个黑块的最少涂色次数

pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let mut w = 0;
    let s = blocks.as_bytes();
    let k = k as usize;
    for i in 0..k {
        if s[i] == b'W' {
            w += 1;
        }
    }
    let mut result = w;
    for i in k..s.len() {
        if s[i] == b'W' {
            w += 1;
        }
        if s[i - k] == b'W' {
            w -= 1;
        }
        result = result.min(w);
    }
    result
}

fn main() {
    assert_eq!(minimum_recolors(String::from("BWWWBB"), 6), 3);
    assert_eq!(minimum_recolors(String::from("WBBWWBBWBW"), 7), 3);
    assert_eq!(minimum_recolors(String::from("WBWBBBW"), 2), 0);
}