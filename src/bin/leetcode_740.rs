//! 删除并获得点数


use std::collections::BTreeMap;

pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
    let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
    for num in nums {
        *cnt.entry(num).or_default() += 1;
    }
    let cnt: Vec<(i32, i32)> = cnt.into_iter().collect();
    let len = cnt.len();
    let mut dp = vec![0; len + 1];
    for i in 1..=len {
        if i > 1 && cnt[i - 1].0 == cnt[i - 2].0 + 1 {
            dp[i] = dp[i - 2] + cnt[i - 1].0 * cnt[i - 1].1;
        } else {
            dp[i] = dp[i - 1] + cnt[i - 1].0 * cnt[i - 1].1;
        }
        dp[i] = dp[i].max(dp[i - 1]);
    }
    dp[len - 1].max(dp[len])
}

fn main() {
    assert_eq!(delete_and_earn(vec![1, 1, 1, 2, 4, 5, 5, 5, 6]), 18);
    assert_eq!(delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    assert_eq!(delete_and_earn(vec![3, 4, 2]), 6);
    assert_eq!(delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
}
