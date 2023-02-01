//! 通知所有员工所需的时间

use std::collections::VecDeque;

pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut children = vec![vec![]; n];
    for (child, parent) in manager.into_iter().enumerate() {
        if child as i32 != head_id {
            children[parent as usize].push(child);
        }
    }
    let mut q = VecDeque::new();
    let mut result = 0;
    q.push_back((head_id as usize, 0));
    while !q.is_empty() {
        let (id, cur_time) = q.pop_front().unwrap();
        result = result.max(cur_time);
        for &child in &children[id] {
            q.push_back((child, cur_time + inform_time[id]));
        }
    }
    result
}

/// dfs
pub fn num_of_minutes2(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    /// 到cur所需的时间, cache: 到cur的parent的时间(能加速。。1s vs 16ms）
    fn dfs(manager: &Vec<i32>, inform_time: &Vec<i32>, cur: usize, cache: &mut Vec<i32>) -> i32 {
        if manager[cur] == -1 {
            cache[cur] = 0;
            return inform_time[cur];
        }
        if cache[cur] != -1 {
            return cache[cur] + inform_time[cur];
        }
        cache[cur] = dfs(manager, inform_time, manager[cur] as usize, cache);
        cache[cur] + inform_time[cur]
    }
    let mut cache = vec![-1; n as usize];
    cache[head_id as usize] = 0;
    (0..n).map(|x| dfs(&manager, &inform_time, x as usize, &mut cache)).max().unwrap()
}

fn main() {
    fn test(func: fn(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32) {
        assert_eq!(func(1, 0, vec![-1], vec![0]), 0);
        assert_eq!(func(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0]), 1);
    }
    test(num_of_minutes);
    test(num_of_minutes2);
}
