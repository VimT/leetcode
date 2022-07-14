//! 金币路径

pub fn cheapest_jump(coins: Vec<i32>, max_jump: i32) -> Vec<i32> {
    let len = coins.len();
    let mut dp = vec![i32::MAX / 2; len];
    let mut next = vec![len; len];
    dp[len - 1] = 0;
    // 因为结果要求字典序，所以要倒序dp，dp[i] 表示从i到 len 需要的最少金额
    // 如果是正序dp，用from数组，那找的是每个节点前面最近的 节点。而字典序是要找后面最近的节点。
    for i in (0..len - 1).rev() {
        if coins[i] != -1 {
            for j in i + 1..(i + max_jump as usize + 1).min(len) {
                if coins[j] > -1 {
                    let cost = dp[j] + coins[i];
                    if cost < dp[i] {
                        dp[i] = cost;
                        next[i] = j;
                    }
                }
            }
        }
    }
    if dp[0] >= i32::MAX / 2 {
        return vec![];
    }
    let mut result = vec![1];
    let mut cur = 0;
    while cur < len - 1 {
        cur = next[cur];
        result.push((cur + 1) as i32);
    }
    result
}

fn main() {
    fn test(func: fn(coins: Vec<i32>, max_jump: i32) -> Vec<i32>) {
        assert_eq!(func(vec![0, 0, 0, 0, 0, 0], 3), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(func(vec![1, 2, 4, -1, 2], 2), vec![1, 3, 5]);
        assert_eq!(func(vec![1, 2, 4, -1, 2], 1), vec![]);
    }
    test(cheapest_jump);
}
