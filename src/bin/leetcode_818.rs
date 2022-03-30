//! 赛车

use std::cmp::Reverse;
use std::collections::BinaryHeap;

// 每次走一个 A*kR
pub fn racecar_dijkstra(target: i32) -> i32 {
    let k = 33 - (target - 1).leading_zeros() as i32;
    let barrier = 1 << k;
    let mut dist = vec![i32::MAX; barrier as usize * 2 + 1];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, target)));
    let len = barrier * 2 + 1;
    dist[target as usize] = 0;
    while !heap.is_empty() {
        let Reverse((step, cur)) = heap.pop().unwrap();
        if dist[((cur + len) % len) as usize] > step {
            continue;
        }
        for i in 0..=k {
            let mut next_step = step + i + 1;
            let next = (1 << i) - 1 - cur;
            if next == 0 { next_step -= 1; }
            if next.abs() <= target && next_step < dist[((next + len) % len) as usize] {
                heap.push(Reverse((next_step, next)));
                dist[((next + len) % len) as usize] = next_step;
            }
        }
    }
    dist[0]
}

static mut MAP: [i32; 10001] = [0; 10001];
static ONCE: std::sync::Once = std::sync::Once::new();

pub fn racecar(target: i32) -> i32 {
    unsafe {
        ONCE.call_once(|| {
            let mut dp = [i32::MAX; 10001];
            dp[0] = 0;
            dp[1] = 1;
            dp[2] = 4;
            for t in 3..=10000_usize {
                let k = 32 - (t as i32).leading_zeros() as i32;
                if t as i32 == (1 << k) - 1 {
                    dp[t] = k;
                    continue;
                }
                for j in 0..k - 1 {
                    dp[t] = dp[t].min(dp[t + (1 << j) - (1 << (k - 1))] + k - 1 + j + 2);
                }
                if (1 << k) - 1 < 2 * t as i32 {
                    dp[t] = dp[t].min(dp[(1 << k) - 1 - t] + k + 1);
                }
            }
            MAP = dp;
        });
        MAP[target as usize]
    }
}


fn main() {
    assert_eq!(racecar(6), 5);
    assert_eq!(racecar(3), 2);
}