//! 屏幕可显示句子的数量

use leetcode::svec;

/// 之前用的是 一遍需要多少行，但是不符合题目特色（短句，长行）。1.2s。
pub fn words_typing(sentence: Vec<String>, rows: i32, cols: i32) -> i32 {
    fn cost(sentence: &Vec<String>, mut j: i32, cols: i32) -> (i32, i32) {
        let mut i = 0;
        for word in sentence {
            if j + word.len() as i32 > cols {
                j = 0;
                i += 1;
            }
            j += word.len() as i32;
            j += 1;
            if j >= cols {
                j = 0;
                i += 1;
            }
        }
        (i, j)
    }
    for word in &sentence {
        if word.len() as i32 > cols { return 0; }
    }
    let mut i = 0;
    let mut j = 0;
    let col_cost: Vec<(i32, i32)> = (0..cols).map(|j| cost(&sentence, j, cols)).collect();
    let mut cnt = 0;
    while i < rows {
        let (di, tj) = col_cost[j as usize];
        i += di;
        if i > rows || (i == rows && tj > 0) { break; }
        j = tj;
        cnt += 1;
    }
    cnt
}

/// 改用一行能有多少遍，4ms
pub fn words_typing2(sentence: Vec<String>, rows: i32, cols: i32) -> i32 {
    // 从第 i 个词开始 这一行能放下几遍句子
    let mut dp = vec![0; sentence.len()];
    // 从第 i 个词开始 放下dp[i]遍句子后 变为第几个词
    let mut next = vec![0; sentence.len()];
    for i in 0..sentence.len() {
        let mut cnt = 0;
        let mut ptr = i;
        let mut cur = cols;
        while cur >= sentence[ptr].len() as i32 {
            cur -= sentence[ptr].len() as i32 + 1;
            ptr += 1;
            if ptr == sentence.len() {
                cnt += 1;
                ptr = 0;
            }
        }
        dp[i] = cnt;
        next[i] = ptr;
    }

    let mut count = 0;
    let mut cur = 0;
    for _ in 0..rows {
        count += dp[cur];
        cur = next[cur];
    }
    return count;
}


fn main() {
    fn test(func: fn(sentence: Vec<String>, rows: i32, cols: i32) -> i32) {
        assert_eq!(func(svec!["hello","leetcode"], 1, 10), 0);
        assert_eq!(func(svec!["hello","world"], 2, 8), 1);
        assert_eq!(func(svec!["a","bcd","e"], 3, 6), 2);
        assert_eq!(func(svec!["i","had","apple","pie"], 4, 5), 1);
    }
    test(words_typing);
    test(words_typing2);
}
