//! 不同骰子序列的数目

pub fn distinct_sequences(n: i32) -> i32 {
    if n == 1 { return 6; }
    if n == 2 { return 22; }
    const MOD: i32 = 1e9 as i32 + 7;
    let can_neigh = vec![vec![1, 2, 3, 4, 5], vec![0, 2, 4], vec![0, 1, 3, 4], vec![0, 2, 4], vec![0, 1, 2, 3, 5], vec![0, 4]];
    let mut dp = [[[0; 6]; 6]; 6];
    for i in 0..6 {
        for &j in &can_neigh[i] {
            for &k in &can_neigh[j] {
                if i != k {
                    dp[i][j][k] = 1;
                }
            }
        }
    }
    for _ in 3..n {
        let mut new_dp = [[[0; 6]; 6]; 6];
        for i in 0..6 {
            for &j in &can_neigh[i] {
                for &k in &can_neigh[j] {
                    if k != i {
                        for &l in &can_neigh[k] {
                            if l != j {
                                new_dp[i][j][k] = (new_dp[i][j][k] + dp[j][k][l]) % MOD;
                            }
                        }
                    }
                }
            }
        }
        dp = new_dp;
    }
    let mut result = 0;
    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                result = (result + dp[i][j][k]) % MOD;
            }
        }
    }
    result
}


fn main() {
    assert_eq!(distinct_sequences(6), 1472);
    assert_eq!(distinct_sequences(4), 184);
    assert_eq!(distinct_sequences(2), 22);
}
