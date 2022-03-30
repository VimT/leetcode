//!  网络延迟时间

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize + 1;
    let mut edges = vec![vec![]; n];
    for time in times {
        edges[time[0] as usize].push((time[1] as usize, time[2]));
    }
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, k as usize)));
    let mut result = 0;
    let mut dp = vec![-1; n];
    while !q.is_empty() {
        let Reverse((cur, node)) = q.pop().unwrap();
        if dp[node] != -1 { continue; }
        dp[node] = cur;
        result = result.max(cur);
        for &(nxt, time) in &edges[node] {
            q.push(Reverse((cur + time, nxt)));
        }
    }
    let mut result = 0;
    for i in 1..n {
        if dp[i] == -1 { return -1; }
        result = result.max(dp[i]);
    }
    result
}

fn main() {
    assert_eq!(network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 3, 4]], 3, 1), 3);
    assert_eq!(network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2), 2);
    assert_eq!(network_delay_time(vec![vec![1, 2, 1]], 2, 1), 1);
    assert_eq!(network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);
}
