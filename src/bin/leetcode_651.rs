//! 4键键盘

pub fn max_a(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        dp[i] = dp[i - 1] + 1;
        for j in 2..i {
            dp[i] = dp[i].max(dp[j - 2] * (i - j + 1));
        }
    }
    dp[n] as i32
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(3), 3);
        assert_eq!(func(7), 9);
    }
    test(max_a);
}
