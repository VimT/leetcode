//! 播放列表的数量

/// dp[i][j] 为播放列表长度为 i 包含恰好 j 首不同歌曲的数量
pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let n = n as usize;
    let l = goal as usize;
    let mut dp = vec![vec![0; n + 1]; l + 1];
    dp[0][0] = 1;
    for i in 1..=l {
        for j in 1..=n {
            dp[i][j] += dp[i - 1][j - 1] * (n as i64 - j as i64 + 1);
            dp[i][j] += dp[i - 1][j] * (j as i64 - k as i64).max(0);
            dp[i][j] %= MOD;
        }
    }

    dp[l][n] as i32
}

fn main() {
    assert_eq!(num_music_playlists(16, 16, 4), 789741546);
    assert_eq!(num_music_playlists(3, 3, 1), 6);
    assert_eq!(num_music_playlists(2, 3, 0), 6);
    assert_eq!(num_music_playlists(2, 3, 1), 2);
}
