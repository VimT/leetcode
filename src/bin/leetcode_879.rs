//! 盈利计划

/// 当只设置 dp[0][0][0] = 1时，结果要把所有dp[len][i][min_profit] 相加（官方题解）
pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let len = group.len();
    let n = n as usize;
    let min_profit = min_profit as usize;
    let mut dp = vec![vec![vec![0; 1 + min_profit]; 1 + n]; len + 1];
    for left_people in 0..=n {
        dp[0][left_people][0] = 1;
    }
    for i in 1..=len {
        for left_people in 0..=n {
            for need_profit in 0..=min_profit {
                let mut t = dp[i - 1][left_people][need_profit];
                if left_people >= group[i - 1] as usize {
                    t += dp[i - 1][left_people - group[i - 1] as usize][(need_profit as i32 - profit[i - 1]).max(0) as usize];
                }
                dp[i][left_people][need_profit] = t % MOD;
            }
        }
    }
    dp[len][n][min_profit] as i32
}

fn main() {
    assert_eq!(profitable_schemes(10, 5, vec![2, 3, 5], vec![6, 7, 8]), 7);
    assert_eq!(profitable_schemes(5, 3, vec![2, 2], vec![2, 3]), 2);
}
