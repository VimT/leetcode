//! 卖木头块

pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
    let m = m as usize;
    let n = n as usize;
    let mut pr = vec![vec![0; n + 1]; m + 1];
    for price in prices.iter() {
        pr[price[0] as usize][price[1] as usize] = price[2];
    }
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = pr[i][j] as i64;
            // 垂直切割
            for k in 1..=j / 2 {
                dp[i][j] = dp[i][j].max(dp[i][k] + dp[i][j - k]);
            }
            // 水平切割
            for k in 1..=i / 2 {
                dp[i][j] = dp[i][j].max(dp[k][j] + dp[i - k][j]);
            }
        }
    }
    dp[m][n]
}

fn main() {
    fn test(func: fn(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64) {
        assert_eq!(func(3, 5, vec![vec![1, 4, 2], vec![2, 2, 7], vec![2, 1, 3]]), 19);
        assert_eq!(func(4, 6, vec![vec![3, 2, 10], vec![1, 4, 2], vec![4, 1, 3]]), 32);
    }
    test(selling_wood);
}
