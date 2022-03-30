//! 火柴拼正方形

use std::collections::HashMap;

/// 如果二进制的第 k 位为 1，那么当前状态包含第 k 根火柴，否则不包含第 k 根火柴
pub fn makesquare(matchsticks: Vec<i32>) -> bool {
    fn recurse(matchsticks: &Vec<i32>, side: i32, mask: u32, mut dones: i32, cache: &mut HashMap<(u32, i32), bool>) -> bool {
        let mut total = 0;
        let len = matchsticks.len();
        let key = (mask, dones);
        for i in (0..len).rev() {
            if mask & (1 << i) == 0 {
                total += matchsticks[len - i - 1];
            }
        }
        if total > 0 && total % side == 0 {
            dones += 1;
        }
        if dones == 3 { return true; }
        if let Some(&v) = cache.get(&key) {
            return v;
        }
        let mut ans = false;
        let c = total / side;
        let rem = side * (c + 1) - total;
        for i in (0..len).rev() {
            if matchsticks[len - i - 1] <= rem && mask & (1 << i) > 0 {
                if recurse(matchsticks, side, mask ^ (1 << i), dones, cache) {
                    ans = true;
                    break;
                }
            }
        }
        cache.insert(key, ans);
        ans
    }
    let len = matchsticks.len();
    if len <= 3 { return false; }
    let sum: i32 = matchsticks.iter().sum();
    if sum % 4 != 0 {
        return false;
    }
    let side = sum / 4;
    let mut cache = HashMap::new();
    return recurse(&matchsticks, side, (1 << len) - 1, 0, &mut cache);
}

pub fn makesquare_dp(mut matchsticks: Vec<i32>) -> bool {
    let len = matchsticks.len();
    let sum: i32 = matchsticks.iter().sum();
    matchsticks.sort_unstable();
    let target = sum / 4;
    if len <= 3 || sum % 4 != 0 || *matchsticks.last().unwrap() > target {
        return false;
    }
    let mut dp = vec![-1; 1 << len];
    dp[0] = 0;
    for i in 1..(1 << len) {
        for j in 0..len {
            let s = i ^ (1 << j);
            if (i >> j & 1) > 0 && dp[s] != -1 {
                let res = target - (dp[s] % target);
                if matchsticks[j] <= res {
                    dp[i] = dp[s] + matchsticks[j];
                    break;
                }
            }
        }
    }
    dp[(1 << len) - 1] != -1
}

fn main() {
    assert!(makesquare_dp(vec![1, 1, 2, 2, 2]));
    assert!(!makesquare_dp(vec![3, 3, 3, 3, 4]));
}
