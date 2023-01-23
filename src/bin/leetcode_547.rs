//! 省份数量

use std::collections::VecDeque;

pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let len = is_connected.len();
    let mut visited = vec![false; len];
    for i in 0..len {
        if !visited[i] {
            ans += 1;
            let mut queue = VecDeque::new();
            queue.push_back(i);
            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();
                for k in 0..len {
                    if is_connected[node][k] != 0 && !visited[k] {
                        visited[node] = true;
                        queue.push_back(k);
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]), 2);
    assert_eq!(find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]), 3);
}
