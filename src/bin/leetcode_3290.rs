//! 最高乘法得分

pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
    let len = b.len();
    let mut dp = vec![[i64::MIN / 2; 5]; len + 1]; // dp[i][j] 表示前i个数，选j个的最大得分
    for i in 0..len { dp[i][0] = 0; }
    for i in 0..len {
        for j in 0..4 {
            dp[i + 1][j + 1] = (dp[i][j] + a[j] as i64 * b[i] as i64).max(dp[i][j + 1]);
        }
    }

    dp[len][4]
}

fn main() {
    fn test(func: fn(a: Vec<i32>, b: Vec<i32>) -> i64) {
        assert_eq!(func(vec![-7, 5, -10, -10], vec![7, -8, -1, 2, 4, 8, -5, -5, 5, -2, 4]), 196);
        assert_eq!(func(vec![3, 2, 5, 6], vec![2, -6, 4, -5, -3, 2, -7]), 26);
        assert_eq!(func(vec![-1, 4, 5, -2], vec![-5, -1, -3, -2, -4]), -1);
    }
    test(max_score);
}
