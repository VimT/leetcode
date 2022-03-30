//! 统计元音字母序列的数目

pub fn count_vowel_permutation(n: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut dp = [1; 5];
    // a e i o u
    for _ in 1..n {
        let mut new_dp = [0; 5];
        // 每个元音 'a' 后面都只能跟着 'e'
        new_dp[1] = dp[0];
        // 每个元音 'e' 后面只能跟着 'a' 或者是 'i'
        new_dp[0] += dp[1];
        new_dp[2] += dp[1];
        // 每个元音 'i' 后面 不能 再跟着另一个 'i'
        new_dp[0] += dp[2];
        new_dp[1] += dp[2];
        new_dp[3] += dp[2];
        new_dp[4] += dp[2];
        // 每个元音 'o' 后面只能跟着 'i' 或者是 'u'
        new_dp[2] += dp[3];
        new_dp[4] += dp[3];
        // 每个元音 'u' 后面只能跟着 'a'
        new_dp[0] += dp[4];
        dp = new_dp;
        for i in 0..5 {
            dp[i] = dp[i] % MOD;
        }
    }
    (dp.iter().sum::<i64>() % MOD) as i32
}

fn main() {
    assert_eq!(count_vowel_permutation(1), 5);
    assert_eq!(count_vowel_permutation(2), 10);
    assert_eq!(count_vowel_permutation(5), 68);
}
