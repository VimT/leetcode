//! 施咒的最大总伤害

use std::collections::HashMap;

/// 和740一样
pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for p in power {
        *cnt.entry(p).or_default() += 1;
    }
    let mut a: Vec<i32> = cnt.keys().copied().collect();
    a.sort_unstable();

    let mut dp = vec![0; a.len() + 1];
    let mut j = 0;
    for (i, &x) in a.iter().enumerate() {
        while a[j] < x - 2 {
            j += 1;
        }
        dp[i + 1] = dp[i].max(dp[j] + x as i64 * cnt[&x] as i64);
    }
    dp[a.len()]
}

fn main() {
    fn test(func: fn(power: Vec<i32>) -> i64) {
        assert_eq!(func(vec![7, 1, 6, 3]), 10);
        assert_eq!(func(vec![1, 1, 3, 4]), 6);
        assert_eq!(func(vec![7, 1, 6, 6]), 13);
        assert_eq!(func(vec![1, 2, 3]), 3);
    }
    test(maximum_total_damage);
}
