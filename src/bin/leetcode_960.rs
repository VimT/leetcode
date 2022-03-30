//! 删列造序 III

use leetcode::svec;

/// dp[i] 表示以i－1结尾的字符的 最小删除列
pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let m = strs.len();
    let n = strs[0].len();
    let mut dp = vec![i32::MAX; n + 1];
    dp[0] = 0;
    for j in 1..=n {
        for k in 0..j {
            let mut ok = true;
            for i in 0..m {
                if k > 0 && strs[i].as_bytes()[j - 1] < strs[i].as_bytes()[k - 1] {
                    ok = false;
                    break;
                }
            }
            if ok {
                dp[j] = dp[j].min(dp[k] + (j - k - 1) as i32);
            }
        }
    }
    let mut result = i32::MAX;
    for i in 1..=n {
        result = result.min(dp[i] + (n - i) as i32);
    }
    result
}

fn main() {
    assert_eq!(min_deletion_size(svec!["babca", "bbazb"]), 3);
    assert_eq!(min_deletion_size(svec!["edcba"]), 4);
    assert_eq!(min_deletion_size(svec!["ghi", "def", "abc"]), 0);
}
