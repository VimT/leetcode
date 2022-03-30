//! 转化数字的最小运算数

use std::collections::VecDeque;

pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
    if start == goal { return 0; }
    let mut vis = vec![false; 1001];
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    while !q.is_empty() {
        let (num, step) = q.pop_front().unwrap();
        if vis[num as usize] {
            continue;
        }
        vis[num as usize] = true;
        for &i in &nums {
            let nxt = [num + i, num - i, num ^ i];
            for i in nxt {
                if i == goal {
                    return step + 1;
                }
                if i >= 0 && i <= 1000 {
                    q.push_back((i, step + 1));
                }
            }
        }
    }

    -1
}


fn main() {
    assert_eq!(minimum_operations(vec![3, 5, 7], 0, -4), 2);
    assert_eq!(minimum_operations(vec![1, 3], 6, 4), 2);
    assert_eq!(minimum_operations(vec![2, 4, 12], 2, 12), 2);
    assert_eq!(minimum_operations(vec![2, 8, 16], 0, 1), -1);
    assert_eq!(minimum_operations(vec![1], 0, 3), 3);
}
