//! 每日温度

use std::collections::VecDeque;

/// 单调栈
pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let mut stack = VecDeque::new();
    let mut ans = vec![0; t.len()];
    for i in 0..t.len() {
        while !stack.is_empty() && t[*stack.back().unwrap()] < t[i] {
            let pop = stack.pop_back().unwrap();
            ans[pop] = (i - pop) as i32;
        }
        stack.push_back(i);
    }
    ans
}

fn main() {
    assert_eq!(daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]), vec![1, 1, 4, 2, 1, 1, 0, 0]);
    assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
    assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
}
