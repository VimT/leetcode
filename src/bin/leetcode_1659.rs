//! 最大化网格幸福感

/// 轮廓线DP
/// 只有前 nn 个格子可能与当前格子发生“相互作用”
pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    let (a, b) = (introverts_count as usize, extroverts_count as usize);
    // 只保存一行的状态
    let status_len = 3usize.pow(n as u32);
    let status_len1 = status_len / 3;
    let mut dp = vec![vec![vec![vec![0; status_len]; b + 1]; a + 1]; n * m + 1];
    let offset = [[0, 0, 0], [0, -60, -10], [0, -10, 40]];
    for c in (0..m * n).rev() {
        let j = c % n;
        for x in 0..=a {
            for y in 0..=b {
                for pre in 0..status_len { // pre 就是前 n 个格子的状态（三进制）
                    let nem = (pre * 3) % status_len; // nem 是 pre “左移” 一位, 并去掉首位的状态,比如三进制 2121->三进制 1210.
                    if x > 0 {
                        let diff = 120 + if j != 0 { offset[1][pre % 3] } else { 0 } + offset[1][pre / status_len1];
                        dp[c][x][y][pre] = dp[c][x][y][pre].max(diff + dp[c + 1][x - 1][y][nem + 1]);
                    }
                    if y > 0 {
                        let diff = 40 + if j != 0 { offset[2][pre % 3] } else { 0 } + offset[2][pre / status_len1];
                        dp[c][x][y][pre] = dp[c][x][y][pre].max(diff + dp[c + 1][x][y - 1][nem + 2]);
                    }
                    dp[c][x][y][pre] = dp[c][x][y][pre].max(dp[c + 1][x][y][nem]);
                }
            }
        }
    }
    dp[0][a][b][0]
}

fn main() {
    assert_eq!(get_max_grid_happiness(2, 3, 1, 2), 240);
    assert_eq!(get_max_grid_happiness(3, 1, 2, 1), 260);
    assert_eq!(get_max_grid_happiness(2, 2, 4, 0), 240);
}
