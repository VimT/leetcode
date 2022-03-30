//! K 站中转内最便宜的航班


use std::collections::VecDeque;

/// 用 f[t][i] 表示通过恰好 t 次航班，从出发城市 src 到达城市 i 需要的最小花费
pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let mut m = vec![vec![]; n as usize];
    for flight in flights {
        m[flight[1] as usize].push((flight[0], flight[2]));
    }
    let n = n as usize;
    let k = k as usize;
    let mut dp = vec![i32::MAX / 2; n];
    dp[src as usize] = 0;
    for _ in 1..=k + 1 {
        let mut new_dp = dp.clone();
        for city in 0..n {
            for &(from, price) in &m[city] {
                new_dp[city] = new_dp[city].min(dp[from as usize] + price);
            }
        }
        dp = new_dp;
    }
    if dp[dst as usize] >= i32::MAX / 2 { -1 } else { dp[dst as usize] }
}

pub fn find_cheapest_price_bfs(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let n = n as usize;
    let mut m = vec![vec![]; n];
    for flight in flights {
        m[flight[0] as usize].push((flight[1], flight[2]));
    }
    let mut q = VecDeque::new();
    q.push_back((src, 0, 0));
    let mut cost = vec![i32::MAX; n];
    while !q.is_empty() {
        let (node, cnt, cur_cost) = q.pop_front().unwrap();
        if cur_cost >= cost[node as usize] {
            continue;
        }
        cost[node as usize] = cur_cost;
        if cnt == k + 1 { continue; }
        for &(nxt, price) in &m[node as usize] {
            if cur_cost + price < cost[nxt as usize] {
                q.push_back((nxt, cnt + 1, cur_cost + price));
            }
        }
    }
    if cost[dst as usize] == i32::MAX { -1 } else { cost[dst as usize] }
}

fn main() {
    assert_eq!(find_cheapest_price_bfs(3, vec![vec![0, 1, 2], vec![1, 2, 1], vec![2, 0, 10]], 1, 2, 1), 1);
    assert_eq!(find_cheapest_price_bfs(3, vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]], 0, 2, 1), 200);
    assert_eq!(find_cheapest_price_bfs(3, vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]], 0, 2, 0), 500);
}
