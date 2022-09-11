//! 预算内的最多机器人数目


use std::collections::VecDeque;

/// 单调队列
pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
    let len = charge_times.len();
    let mut result = 0;
    let mut q = VecDeque::new();
    let mut tot = 0; // 窗口内的runningCosts子数组和
    let mut k = 0; // 窗口长度

    for i in 0..len {
        while !q.is_empty() && charge_times[*q.back().unwrap()] <= charge_times[i] {
            q.pop_back();
        }
        // 增大窗口，队尾入队
        q.push_back(i);
        tot += running_costs[i] as i64;
        k += 1;

        // 判断是否满足条件，队首出队
        while !q.is_empty() && charge_times[*q.front().unwrap()] as i64 + k * tot > budget {
            k -= 1;
            tot -= running_costs[i - k as usize] as i64;
            if q[0] == i - k as usize {
                q.pop_front();
            }
        }
        result = result.max(k);
    }

    result as i32
}

fn main() {
    assert_eq!(maximum_robots(vec![11, 12, 19], vec![10, 8, 7], 19), 0);
    assert_eq!(maximum_robots(vec![3, 6, 1, 3, 4], vec![2, 1, 3, 4, 5], 25), 3);
}
