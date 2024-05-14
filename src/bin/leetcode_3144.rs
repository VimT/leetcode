//! 分割字符频率相等的最少子字符串

pub fn minimum_substrings_in_partition(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![i32::MAX; len + 1]; // 前 i 个字符的最少分割
    dp[0] = 0;
    for i in 0..len {
        let mut cnt = vec![0; 26];
        let mut mx = 0;
        let mut has_num = 0;
        let mut mx_num = 0;
        for j in (0..=i).rev() {
            let idx = (s[j] - b'a') as usize;
            cnt[idx] += 1;
            if cnt[idx] > mx {
                mx = cnt[idx];
                mx_num = 1;
            } else if cnt[idx] == mx {
                mx_num += 1;
            }
            if cnt[idx] == 1 { has_num += 1; }
            if has_num == mx_num { dp[i + 1] = dp[i + 1].min(dp[j] + 1); }
        }
    }
    dp[len]
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("fabccddg")), 3);
        assert_eq!(func(String::from("abababaccddb")), 2);
    }
    test(minimum_substrings_in_partition);
}
