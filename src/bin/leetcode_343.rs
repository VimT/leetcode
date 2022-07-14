//! 整数拆分

pub fn integer_break(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![1; n + 1];

    for i in 2..=n {
        for k in 1..=i - 1 {
            dp[i] = dp[i].max(dp[i - k] * k).max((i - k) * k);
        }
    }
    dp[n] as i32
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(2), 1);
        assert_eq!(func(10), 36);
    }
    test(integer_break);
}
