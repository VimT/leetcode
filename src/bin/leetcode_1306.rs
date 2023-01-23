//! 跳跃游戏 III

use std::collections::VecDeque;

pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    let len = arr.len();
    let mut seen = vec![false; len];
    let mut q = VecDeque::new();
    seen[start as usize] = true;
    q.push_back(start as usize);
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        if arr[u] == 0 {
            return true;
        }
        let a = u + (arr[u] as usize);
        if a < len && !seen[a]{
            seen[a] = true;
            q.push_back(a);
        }
        if u >= arr[u] as usize {
            let b = u - (arr[u] as usize);
            if !seen[b] {
                seen[b] = true;
                q.push_back(b);
            }
        }
    }
    false
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, start: i32) -> bool) {
        assert_eq!(func(vec![4, 2, 3, 0, 3, 1, 2], 5), true);
        assert_eq!(func(vec![4, 2, 3, 0, 3, 1, 2], 0), true);
        assert_eq!(func(vec![3, 0, 2, 1, 2], 2), false);
    }
    test(can_reach);
}
