//! 分割回文串 III


/// need[i][j] 表示[i,j] 变成回文串需要多少次修改（区间动态规划）
/// dp[k][i] 前i个字符串分割k次需要修改多少次。
pub fn palindrome_partition(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut need = vec![vec![0; len]; len];
    for r in 2..=len {
        for i in 0..=len - r {
            let j = i + r - 1;
            need[i][j] = need[i + 1][j - 1] + (if s[i] == s[j] { 0 } else { 1 });
        }
    }
    let mut dp = vec![vec![i32::MAX / 2; len + 1]; k as usize + 1];
    dp[0][0] = 0;
    for i in 1..=k as usize {
        for j in 1..=len {
            for x in 0..j {
                dp[i][j] = dp[i][j].min(dp[i - 1][x] + need[x][j - 1]);
            }
        }
    }
    dp[k as usize][len]
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("abc"), 2), 1);
        assert_eq!(func(String::from("aabbc"), 3), 0);
        assert_eq!(func(String::from("leetcode"), 8), 0);
    }
    test(palindrome_partition);
}
