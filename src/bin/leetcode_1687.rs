//! 从仓库到码头运输箱子

use std::collections::VecDeque;

pub fn box_delivering(boxes: Vec<Vec<i32>>, _: i32, max_boxes: i32, max_weight: i32) -> i32 {
    let len = boxes.len();
    let mut presum = vec![0; len + 1];
    let mut port_count_presum = vec![0; len + 1];
    for i in 1..=len {
        presum[i] = boxes[i - 1][1] + presum[i - 1];
        port_count_presum[i] = port_count_presum[i - 1];
        if i < len && boxes[i - 1][0] != boxes[i][0] {
            port_count_presum[i] += 1;
        }
    }
    let mut dp = vec![0; len + 1];
    let mut q = VecDeque::new();
    q.push_back(0);
    for i in 1..=len {
        while !q.is_empty() && (presum[i] - presum[*q.front().unwrap()] > max_weight || i - *q.front().unwrap() > max_boxes as usize) {
            q.pop_front().unwrap();
        }
        let start = *q.front().unwrap();
        dp[i] = (dp[i - 1] + 2).min(dp[start] + port_count_presum[i - 1] - port_count_presum[start] + 2);
        while !q.is_empty() && (dp[i] - port_count_presum[i] <= dp[*q.back().unwrap()] - port_count_presum[*q.back().unwrap()]) {
            q.pop_back().unwrap();
        }
        q.push_back(i);
    }

    dp[len]
}

fn main() {
    assert_eq!(box_delivering(vec![vec![1, 1], vec![2, 1], vec![1, 1]], 2, 3, 3), 4);
    assert_eq!(box_delivering(vec![vec![2, 4], vec![2, 5], vec![3, 1], vec![3, 2], vec![3, 7], vec![3, 1], vec![4, 4], vec![1, 3], vec![5, 2]], 5, 5, 7), 14);
    assert_eq!(box_delivering(vec![vec![1, 2], vec![3, 3], vec![3, 1], vec![3, 1], vec![2, 4]], 3, 3, 6), 6);
    assert_eq!(box_delivering(vec![vec![1, 4], vec![1, 2], vec![2, 1], vec![2, 1], vec![3, 2], vec![3, 4]], 3, 6, 7), 6);
}
