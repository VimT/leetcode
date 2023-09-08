//! 大小为 K 的不重叠线段的数目

/// dp[i][j][0/1] 表示前i个点，构造k个线段的方案数，线段有没有用最后一个点
/// 还可以数学法： C(n+k−1, 2k)
pub fn number_of_sets(n: i32, k: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let n = n as usize;
    let k = k as usize;
    let mut dp = vec![vec![[0; 2]; k + 1]; n];
    for i in 0..n {
        dp[i][0][0] = 1;
        dp[i][0][1] = 0;
    }
    for i in 1..n {
        for j in 1..=k {
            dp[i][j][0] = (dp[i - 1][j][0] + dp[i - 1][j][1]) % MOD;
            dp[i][j][1] = ((dp[i - 1][j][1] + dp[i - 1][j - 1][1]) % MOD + dp[i - 1][j - 1][0]) % MOD;
        }
    }
    (dp[n - 1][k][0] + dp[n - 1][k][1]) % MOD
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> i32) {
        assert_eq!(func(4, 2), 5);
        assert_eq!(func(3, 1), 3);
        assert_eq!(func(30, 7), 796297179);
        assert_eq!(func(5, 3), 7);
        assert_eq!(func(3, 2), 1);
    }
    test(number_of_sets);
}
