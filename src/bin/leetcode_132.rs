//! 分割回文串 II


/// dp[i] = min(dp[j] + 1 if s[j + 1: i] is palindrome for j in range(i))
pub fn min_cut(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    if len < 2 { return 0; }
    let mut dp = vec![0; len];
    for i in 0..len {
        dp[i] = i;
    }

    let mut pdp = vec![vec![false; len]; len];
    for right in 0..len {
        for left in 0..=right {
            pdp[left][right] = s[right] == s[left] && (right - left <= 2 || pdp[left + 1][right - 1]);
        }
    }

    for i in 1..len {
        if pdp[0][i] {
            dp[i] = 0;
            continue;
        }
        for j in 0..i {
            if pdp[j + 1][i] {
                dp[i] = dp[i].min(dp[j] + 1);
            }
        }
    }

    dp[len - 1] as i32
}


fn main() {
    assert_eq!(min_cut(String::from("aab")), 1);
    assert_eq!(min_cut(String::from("a")), 0);
    assert_eq!(min_cut(String::from("ab")), 1);
}
