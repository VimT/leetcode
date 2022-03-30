//! 不含连续1的非负整数


pub fn find_integers(n: i32) -> i32 {
    let mut dp = vec![1; 31];
    for i in 2..31 {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    let mut pre = 0;
    let mut res = 0;
    for i in (0..=29).rev() {
        if n & (1 << i) != 0 {
            res += dp[i + 1];
            if pre == 1 { break; }
            pre = 1;
        } else {
            pre = 0;
        }
        if i == 0 {
            res += 1;
        }
    }
    res
}

fn main() {
    assert_eq!(find_integers(5), 5);
}