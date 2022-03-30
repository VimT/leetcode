//! K个逆序对数组

pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
    const YU: i32 = 1e9 as i32 + 7;
    let k = k as usize;
    let n = n as usize;
    let mut dp = vec![vec![0; k + 1]; 2];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 0..=k {
            let cur = i & 1;
            let prev = cur ^ 1;
            dp[cur][j] = if j >= 1 { dp[cur][j - 1] } else { 0 } - if j >= i { dp[prev][j - i] } else { 0 } + dp[prev][j];
            if dp[cur][j] >= YU {
                dp[cur][j] -= YU;
            } else if dp[cur][j] < 0 {
                dp[cur][j] += YU;
            }
        }
    }
    dp[n & 1][k]
}

fn main() {
    assert_eq!(k_inverse_pairs(2, 2), 0);
    assert_eq!(k_inverse_pairs(3, 0), 1);
    assert_eq!(k_inverse_pairs(3, 1), 2);
}
