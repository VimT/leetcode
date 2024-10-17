//! 从原字符串里进行删除操作的最多次数

pub fn max_removals(source: String, pattern: String, target_indices: Vec<i32>) -> i32 {
    let s = source.as_bytes();
    let p = pattern.as_bytes();
    let n = s.len();
    let mut del = vec![false; n];
    for t in target_indices { del[t as usize] = true; }
    let m = pattern.len();
    // dp[i][j] 表示 pattern[..j] 是 pattern[..i] 的字序列，最多可以做多少次删除
    let mut dp = vec![vec![i32::MIN; m + 1]; n + 1];
    dp[0][0] = 0;
    for (i, &ch) in s.iter().enumerate() {
        let is_del = del[i] as i32;
        dp[i + 1][0] = dp[i][0] + is_del;
        for j in 0..(i + 1).min(m) {
            let mut a = dp[i][j + 1] + is_del;
            if ch == p[j] { a = a.max(dp[i][j]); }
            dp[i + 1][j + 1] = a;
        }
    }
    dp[n][m]
}

fn main() {
    fn test(func: fn(source: String, pattern: String, target_indices: Vec<i32>) -> i32) {
        assert_eq!(func(String::from("abbaa"), String::from("aba"), vec![0, 1, 2]), 1);
        assert_eq!(func(String::from("bcda"), String::from("d"), vec![0, 3]), 2);
        assert_eq!(func(String::from("dda"), String::from("dda"), vec![0, 1, 2]), 0);
        assert_eq!(func(String::from("yeyeykyded"), String::from("yeyyd"), vec![0, 2, 3, 4]), 2);
    }
    test(max_removals);
}
