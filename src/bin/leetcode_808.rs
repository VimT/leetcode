//! 分汤

pub fn soup_servings(n: i32) -> f64 {
    let n = (n / 25 + if n % 25 == 0 { 0 } else { 1 }) as usize;
    // !!!
    if n >= 500 { return 1.0; }
    let mut dp = vec![vec![0.; n + 1]; n + 1];
    #[inline]
    fn m(u: usize, sub: usize) -> usize {
        if u >= sub { return u - sub; } else { 0 }
    }
    for s in 0..=2 * n {
        for i in 0..=n {
            if s < i || s - i > n { continue; }
            let j = s - i;
            dp[i][j] = if i == 0 && j == 0 {
                0.5
            } else if i == 0 {
                1.
            } else if i > 0 && j > 0 {
                0.25 * (dp[m(i, 4)][j] + dp[m(i, 3)][m(j, 1)] + dp[m(i, 2)][m(j, 2)] + dp[m(i, 1)][m(j, 3)])
            } else {
                0.
            };
        }
    }
    dp[n][n]
}

fn main() {
    assert_eq!(soup_servings(50), 0.62500);
    assert_eq!(soup_servings(100), 0.71875);
}
