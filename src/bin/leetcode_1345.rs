//! 跳跃游戏 IV

use std::collections::{HashMap, VecDeque};

pub fn min_jumps(arr: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, v) in arr.iter().enumerate() {
        map.entry(*v).or_default().push(i);
    }
    let len = arr.len();
    let mut vis = vec![false; len];
    let mut q = VecDeque::with_capacity(len);
    q.push_back((0, 0));
    vis[0] = true;
    while !q.is_empty() {
        let (node, step) = q.pop_front().unwrap();
        if node == len - 1 {
            return step;
        }
        if map.contains_key(&arr[node]) {
            for &k in &map[&arr[node]] {
                if !vis[k] {
                    vis[k] = true;
                    q.push_back((k, step + 1));
                }
            }
            map.remove(&arr[node]);
        }
        if node > 0 && !vis[node - 1] {
            vis[node - 1] = true;
            q.push_back((node - 1, step + 1));
        }
        if !vis[node + 1] {
            vis[node + 1] = true;
            q.push_back((node + 1, step + 1));
        }
    }
    unreachable!()
}

fn main() {
    assert_eq!(min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]), 3);
    assert_eq!(min_jumps(vec![7]), 0);
    assert_eq!(min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
}
