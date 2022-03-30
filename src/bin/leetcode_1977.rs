//! 划分数字的方案数

const YU: u64 = 1e9 as u64 + 7;

pub fn number_of_combinations(num: String) -> i32 {
    let num = num.as_bytes();
    let len = num.len();
    if num[0] == b'0' { return 0; }
    let mut dp = vec![vec![0; len + 1]; len + 1];
    for i in 1..=len {
        dp[i][i] = 1;
    }
    let mut done = vec![false; len + 1];
    let mut dpsum = vec![vec![0; len + 1]; len + 1];
    for i in 2..=len {
        for j in 1..i {
            if num[i - j] == b'0' { continue; }
            // the compare can optimised by lcp
            if i >= 2 * j && num[i - 2 * j] != b'0' && num[i - 2 * j..i - j] <= num[i - j..i] {
                dp[i][j] += dp[i - j][j];
            }
            if !done[i - j] {
                for k in 1..=len {
                    dpsum[i - j][k] = dpsum[i - j][k - 1] + dp[i - j][k];
                }
                done[i - j] = true;
            }
            dp[i][j] += dpsum[i - j][j - 1];
            dp[i][j] %= YU;
        }
    }
    (dp[len].iter().sum::<u64>() % YU) as i32
}

fn main() {
    assert_eq!(number_of_combinations(String::from("327")), 2);
    assert_eq!(number_of_combinations(String::from("094")), 0);
    assert_eq!(number_of_combinations(String::from("0")), 0);
}
