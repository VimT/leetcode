//! 钥匙和房间

use std::collections::VecDeque;

pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let len = rooms.len();
    let mut vis = vec![false; len];
    let mut q = VecDeque::new();
    vis[0] = true;
    q.push_back(0);
    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        for &next in &rooms[cur] {
            if !vis[next as usize] {
                vis[next as usize] = true;
                q.push_back(next as usize);
            }
        }
    }
    for i in vis {
        if !i { return false; }
    }
    true
}

fn main() {
    assert_eq!(can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]), true);
    assert_eq!(can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]), false);
}
