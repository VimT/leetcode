//! 二指输入的的最小距离

fn dis(a: u8, b: u8) -> i32 {
    if a == 26 { return 0; }
    let (x1, y1) = (a / 6, a % 6);
    let (x2, y2) = (b / 6, b % 6);
    (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()
}

/// 记忆化搜索
pub fn minimum_distance(word: String) -> i32 {
    let s = word.as_bytes();
    fn dfs(s: &[u8], i: usize, l: u8, r: u8, cache: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if i == s.len() { return 0; }
        if cache[i][l as usize][r as usize] != -1 { return cache[i][l as usize][r as usize]; }
        let pos = s[i] - b'A';
        let result = (dfs(s, i + 1, pos, r, cache) + dis(l, pos)).min(dfs(s, i + 1, l, pos, cache) + dis(r, pos));
        cache[i][l as usize][r as usize] = result;
        result
    }
    dfs(s, 0, 26, 26, &mut vec![vec![vec![-1; 27]; 27]; s.len()])
}

/// 记忆化搜索转为 dp[i][j][k]  当前第i个字母，左手j右手k
pub fn minimum_distance2(word: String) -> i32 {
    let s = word.as_bytes();
    let len = s.len();
    let mut dp = vec![vec![vec![i32::MAX / 2; 26]; 26]; len];
    for i in 0..26 {
        dp[0][i][(s[0] - b'A') as usize] = 0;
        dp[0][(s[0] - b'A') as usize][i] = 0;
    }
    for i in 1..len {
        let cur = (s[i] - b'A') as usize;
        let prev = (s[i - 1] - b'A') as usize;
        let d = dis(prev as u8, cur as u8);
        // 移动同一个手指
        for j in 0..26 {
            dp[i][cur][j] = dp[i][cur][j].min(dp[i - 1][prev][j] + d);
            dp[i][j][cur] = dp[i][j][cur].min(dp[i - 1][j][prev] + d);
        }
        for j in 0..26 {
            let d = dis(j as u8, cur as u8);
            dp[i][cur][prev] = dp[i][cur][prev].min(dp[i - 1][j][prev] + d);
            dp[i][prev][cur] = dp[i][prev][cur].min(dp[i - 1][prev][j] + d);
        }
    }
    let mut result = i32::MAX;
    for i in 0..26 {
        for j in 0..26 {
            result = result.min(dp[len - 1][i][j]);
        }
    }
    result
}

fn main() {
    fn test(func: fn(word: String) -> i32) {
        assert_eq!(func(String::from("CAKE")), 3);
        assert_eq!(func(String::from("HAPPY")), 6);
    }
    test(minimum_distance);
    test(minimum_distance2);
}
