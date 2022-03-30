//! DI 序列的有效排列

/// dp[i][j] 表示第 i 个数填入后，以j结尾有多少种
/// D -> (1 0)
/// DI -> (2 0 1) (1 0 2)  // will insert 1/2, 插入1的时候，把前面所有>=1 的都+1
/// DID -> will insert 0/1/2   插入的时候，要<=前面的数。
///   插入0：(3 1 2 0) (2 1 3 0)
///   插入1：(3 0 2 1) (2 0 3 1)
///   插入2：(1 0 3 2)
pub fn num_perms_di_sequence(s: String) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![vec![0; len + 1]; len + 1];
    dp[0].fill(1);
    for i in 1..=len {
        for j in 0..=i {
            let mut t = 0;
            let (start, end) = if s[i - 1] == b'D' {
                (j, i)
            } else { (0, j) };
            for k in start..end {
                t = (t + dp[i - 1][k]) % MOD;
            }
            dp[i][j] = t;
        }
    }
    let mut result = 0;
    for &x in &dp[len] {
        result = (result + x) % MOD;
    }
    result
}

fn main() {
    assert_eq!(num_perms_di_sequence(String::from("DID")), 5);
    assert_eq!(num_perms_di_sequence(String::from("D")), 1);
}
