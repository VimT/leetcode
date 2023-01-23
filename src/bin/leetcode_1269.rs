//! 停在原地的方案数

/// dp[i][j] 表示当前走了 i 步后，位置在 j 有多少种走法
pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let steps = steps as usize;
    let arr_len = (arr_len as usize).min(steps);  // 优化:最远走到 steps处
    let mut dp = vec![0; arr_len];
    dp[0] = 1;
    for _ in 0..steps {
        let mut new_dp = vec![0; arr_len];
        for i in 0..arr_len {
            new_dp[i] = if i > 0 { dp[i - 1] } else { 0 } + dp[i] + if i + 1 < arr_len { dp[i + 1] } else { 0 };
            new_dp[i] %= MOD;
        }
        dp = new_dp;
    }
    dp[0] as i32
}

fn main() {
    fn test(func: fn(steps: i32, arr_len: i32) -> i32) {
        assert_eq!(func(500, 969997), 374847123);
        assert_eq!(func(27, 7), 127784505);
        assert_eq!(func(3, 2), 4);
        assert_eq!(func(2, 4), 2);
        assert_eq!(func(4, 2), 8);
    }
    test(num_ways);
}
