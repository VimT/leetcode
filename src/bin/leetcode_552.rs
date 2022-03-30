//! 学生出勤记录 II

pub fn check_record(n: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let n = n as usize;
    let mut dp = vec![vec![0; 3]; 2];
    dp[0][0] = 1;
    for _ in 1..=n {
        let mut new_dp = vec![vec![0; 3]; 2];
        // p
        for j in 0..=1 {
            for k in 0..=2 {
                new_dp[j][0] = (new_dp[j][0] + dp[j][k]) % MOD;
            }
        }
        // a
        for k in 0..=2 {
            new_dp[1][0] = (new_dp[1][0] + dp[0][k]) % MOD;
        }
        // l
        for j in 0..=1 {
            for k in 1..=2 {
                new_dp[j][k] = (new_dp[j][k] + dp[j][k - 1]) % MOD;
            }
        }
        dp = new_dp;
    }
    let mut result = 0;
    for j in 0..=1 {
        for k in 0..=2 {
            result = (result + dp[j][k]) % MOD;
        }
    }
    result
}

fn main() {
    assert_eq!(check_record(2), 8);
    assert_eq!(check_record(1), 3);
    assert_eq!(check_record(10101), 183236316);
}
