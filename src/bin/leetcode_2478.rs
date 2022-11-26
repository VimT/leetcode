//! 完美分割的方案数

fn is_prime(ch: u8) -> bool {
    matches!(ch, b'2'|b'3'|b'5'|b'7')
}

/// dp[i][j]  表示最后一个切割点以i为开头，切分j次的切割方法有多少种，复杂度 O(n*n*k)
/// 使用前缀和优化： O(n*k)
pub fn beautiful_partitions(s: String, k: i32, min_length: i32) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    if !is_prime(s[0]) || is_prime(s[len - 1]) { return 0; }
    const MOD: i32 = 1e9 as i32 + 7;
    let k = k as usize;
    let min_len = min_length as usize;
    let mut f = vec![vec![0; k + 1]; len + 1];
    let mut g = vec![vec![0; k + 1]; len + 1];
    f[0][0] = 1;
    g[0][0] = 1;
    for i in 1..=len {
        // 第i个字符能否成为分割点
        if i >= min_len && !is_prime(s[i - 1]) && (i == len || is_prime(s[i])) {
            for j in 1..=k {
                f[i][j] = g[i - min_len][j - 1];
            }
        }
        // 维护前缀和
        for j in 0..=k {
            g[i][j] = (g[i - 1][j] + f[i][j]) % MOD;
        }
    }

    f[len][k]
}

fn main() {
    fn test(func: fn(s: String, k: i32, min_length: i32) -> i32) {
        assert_eq!(func(String::from("22"), 1, 1), 0);
        assert_eq!(func(String::from("23542185131"), 3, 2), 3);
        assert_eq!(func(String::from("23542185131"), 3, 3), 1);
        assert_eq!(func(String::from("3312958"), 3, 1), 1);
    }
    test(beautiful_partitions);
}
