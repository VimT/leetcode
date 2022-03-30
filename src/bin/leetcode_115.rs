//! 不同的子序列


pub fn num_distinct(s: String, t: String) -> i32 {
    fn inner(s: &[u8], t: &[u8], s_idx: usize, t_idx: usize, mem: &mut Vec<Vec<i32>>) -> i32 {
        if t_idx == t.len() { return 1; }
        if s_idx == s.len() { return 0; }
        if mem[s_idx][t_idx] != -1 { return mem[s_idx][t_idx]; }
        let c = t[t_idx];
        let mut ans = 0;
        for i in s_idx..s.len() {
            if s[i] == c {
                ans += inner(s, t, i + 1, t_idx + 1, mem);
            }
        }
        mem[s_idx][t_idx] = ans;
        ans
    }
    let s = s.as_bytes();
    let t = t.as_bytes();
    if t.len() == 0 || s.len() == 0 { return 0; }
    inner(s, t, 0, 0, &mut vec![vec![-1; t.len()]; s.len()])
}

pub fn num_distinct_dp(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let t_len = t.len();
    let s_len = s.len();
    if t_len == 0 || s_len == 0 || t_len > s_len { return 0; }
    let mut dp = vec![vec![0; t_len + 1]; s_len + 1];
    for i in 0..=s_len {
        dp[i][t_len] = 1;
    }
    for t_idx in (0..t_len).rev() {
        let c = t[t_idx];
        for s_idx in (t_idx..=s_len + t_idx - t_len).rev() {
            let mut tmp = 0;
            for i in s_idx..=s_len + t_idx - t_len {
                if s[i] == c {
                    tmp += dp[i + 1][t_idx + 1];
                }
            }
            dp[s_idx][t_idx] = tmp;
        }
    }
    dp[0][0]
}

///dp[i][j] 代表 T 前 i 字符串可以由 S j 字符串组成最多个数.
pub fn num_distinct_dpp(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let t_len = t.len();
    let s_len = s.len();
    if t_len == 0 || s_len == 0 || t_len > s_len { return 0; }
    let mut dp = vec![vec![0; s_len + 1]; t_len + 1];
    for i in 0..=s_len {
        dp[0][i] = 1;
    }
    for i in 1..=t_len {
        for j in 1..=s_len {
            dp[i][j] = if t[i - 1] == s[j - 1] {
                dp[i - 1][j - 1] + dp[i][j - 1]
            } else {
                dp[i][j - 1]
            };
        }
    }
    dp[t_len][s_len]
}

fn main() {
    fn test(func: fn(s: String, t: String) -> i32) {
        assert_eq!(func(String::from("rabbbit"), String::from("rabbit")), 3);
        assert_eq!(func(String::from("babgbag"), String::from("bag")), 5);
    }
    test(num_distinct);
    test(num_distinct_dp);
    test(num_distinct_dpp);
}
