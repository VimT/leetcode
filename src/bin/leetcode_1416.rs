//! 恢复数组

pub fn number_of_arrays(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let k_len = k.to_string().len();
    let mut dp = vec![0; len + 1];
    dp[0] = 1;
    let k = k as i64;
    const MOD: i64 = 1e9 as i64 + 7;
    for i in 1..=len {
        let mut cur = 0;
        let mut base = 1;
        for j in 1..=k_len.min(i) {
            if s[i - j] != b'0' {
                cur += (s[i - j] - b'0') as i64 * base;
                if cur <= k {
                    dp[i] += dp[i - j];
                }
            }
            base *= 10;
        }
        dp[i] %= MOD;
    }
    dp[len] as i32
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("1000"), 10000), 1);
        assert_eq!(func(String::from("1000"), 10), 0);
        assert_eq!(func(String::from("1317"), 2000), 8);
    }
    test(number_of_arrays);
}
