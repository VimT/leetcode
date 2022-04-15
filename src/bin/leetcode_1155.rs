//! 掷骰子的N种方法

pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let mut dp = vec![0; target as usize + 1];
    dp[0] = 1;
    for _ in 0..n {
        for i in (0..=target as usize).rev() {
            // 上一轮的数置零
            dp[i] = 0;
            for j in 1..=(k as usize).min(i) {
                dp[i] = (dp[i] + dp[i - j]) % MOD;
            }
        }
    }
    dp[target as usize]
}

fn main() {
    fn test(func: fn(n: i32, k: i32, target: i32) -> i32) {
        assert_eq!(func(1, 6, 3), 1);
        assert_eq!(func(2, 6, 7), 6);
        assert_eq!(func(30, 30, 500), 222616187);
    }
    test(num_rolls_to_target);
}
