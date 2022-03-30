//! 青蛙过河


use std::collections::HashMap;

pub fn can_cross(stones: Vec<i32>) -> bool {
    fn dfs(stones: &Vec<i32>, idx: usize, k: i32, cache: &mut HashMap<(usize, i32), bool>) -> bool {
        if k <= 0 { return false; }
        if idx == stones.len() - 1 {
            return true;
        }
        if let Some(v) = cache.get(&(idx, k)) {
            return *v;
        }
        let mut result = false;
        let cur = stones[idx];
        let target = cur + k - 1;
        let start = stones.binary_search(&target).unwrap_or_else(|x| x);
        'out: for nxt in start.max(idx + 1)..stones.len() {
            if stones[nxt] > cur + k + 1 {
                break;
            }
            for nxt_k in k - 1..=k + 1 {
                if stones[nxt] == cur + nxt_k {
                    if dfs(stones, nxt, nxt_k, cache) {
                        result = true;
                        break 'out;
                    }
                }
            }
        }
        cache.insert((idx, k), result);
        result
    }
    if stones[1] != 1 {
        return false;
    }
    dfs(&stones, 1, 1, &mut HashMap::new())
}

pub fn can_cross_dp(stones: Vec<i32>) -> bool {
    let len = stones.len();
    let mut dp = vec![vec![false; len]; len];
    dp[0][0] = true;
    for i in 1..len {
        // 青蛙所在的石子编号至少增加 1，而每次跳跃距离至多增加 1。
        if stones[i] - stones[i - 1] > i as i32 {
            return false;
        }
    }
    for i in 1..len {
        for j in (0..i).rev() {
            let k = (stones[i] - stones[j]) as usize;
            if k > j + 1 {
                break;
            }
            dp[i][k] = dp[j][k - 1] || dp[j][k] || dp[j][k + 1];
            if i == len - 1 && dp[i][k] {
                return true;
            }
        }
    }
    false
}


fn main() {
    fn test(func: fn(stones: Vec<i32>) -> bool) {
        assert_eq!(func(vec![0, 1, 3, 5, 6, 8, 12, 17]), true);
        assert_eq!(func(vec![0, 1, 2, 3, 4, 8, 9, 11]), false);
    }
    test(can_cross);
    test(can_cross_dp);
}
