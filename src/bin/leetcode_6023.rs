//! 用地毯覆盖后的最少白色砖块

pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
    let s = floor.as_bytes();
    let num_carpets = num_carpets as usize;
    let carpet_len = carpet_len as usize;
    let len = s.len();
    if num_carpets * carpet_len >= len {
        return 0;
    }
    fn dfs(s: &[u8], pre: &Vec<i32>, cl: usize, mut idx: usize, left: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
        if idx >= s.len() || left == 0 {
            return 0;
        }
        if cache[idx][left] != -1 {
            return cache[idx][left];
        }
        while idx < s.len() && s[idx] == b'0' { idx += 1; }
        if idx == s.len() { return 0; }
        let result = (pre[(idx + cl).min(s.len())] - pre[idx] + dfs(s, pre, cl, idx + cl, left - 1, cache)).max(dfs(s, pre, cl, idx + 1, left, cache));
        cache[idx][left] = result;
        return result;
    }
    let mut dp = vec![vec![-1; num_carpets + 1]; len];
    let mut pre = vec![0; len + 1];
    for i in 0..len {
        pre[i + 1] = pre[i] + if s[i] == b'1' { 1 } else { 0 };
    }
    pre[s.len()] - dfs(s, &pre, carpet_len, 0, num_carpets, &mut dp)
}

/// dp[i][j] 表示前 i 个砖块，使用 j 个毯子铺，最多能铺多少白色砖块
/// dp[i][j] = max(dp[i-1][j] (不铺), dp[i-carpet_len][j-1] (铺))
pub fn minimum_white_tiles_dp(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
    let s = floor.as_bytes();
    let num_carpets = num_carpets as usize;
    let carpet_len = carpet_len as usize;
    let len = s.len();
    if num_carpets * carpet_len >= len {
        return 0;
    }
    let mut dp = vec![vec![0; num_carpets + 1]; len + 1];
    let mut pre = vec![0; len + 1];
    for i in 0..len {
        pre[i + 1] = pre[i] + if s[i] == b'1' { 1 } else { 0 };
    }
    for i in 1..=len {
        for j in 1..=num_carpets {
            dp[i][j] = dp[i - 1][j];
            if s[i - 1] == b'1' {
                let start = if i < carpet_len { 0 } else { i - carpet_len };
                dp[i][j] = dp[i][j].max(dp[start][j - 1] + pre[i] - pre[start]);
                // println!("{},{} = {}", i, j, dp[i][j]);
            }
        }
    }
    pre[s.len()] - dp[len][num_carpets]
}


fn main() {
    fn test(func: fn(floor: String, num_carpets: i32, carpet_len: i32) -> i32) {
        assert_eq!(func(String::from("0000"), 1, 1), 0);
        assert_eq!(func(String::from("11010110"), 12, 4), 0);
        assert_eq!(func(String::from("11111"), 2, 3), 0);
        assert_eq!(func(String::from("10110101"), 2, 2), 2);
    }
    test(minimum_white_tiles);
    test(minimum_white_tiles_dp);
}
