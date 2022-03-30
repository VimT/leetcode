//! 一和零

use leetcode::svec;

pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let len = strs.len();
    let zo: Vec<(usize, usize)> = strs.into_iter().map(|x| {
        let s = x.as_bytes();
        let mut zero = 0;
        let mut one = 0;
        for &ch in s {
            if ch == b'0' { zero += 1 } else { one += 1 }
        }
        (zero, one)
    }).collect();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..len {
        for j in (zo[i].0..=m).rev() {
            for k in (zo[i].1..=n).rev() {
                dp[j][k] = dp[j][k].max(dp[j - zo[i].0][k - zo[i].1] + 1);
            }
        }
    }
    dp[m][n]
}

fn main() {
    assert_eq!(find_max_form(svec!["10", "0001", "111001", "1", "0"], 5, 3), 4);
    assert_eq!(find_max_form(svec!["10", "0", "1"], 1, 1), 2);
}
