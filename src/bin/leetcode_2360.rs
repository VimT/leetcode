//! 图中的最长环

use std::collections::VecDeque;

pub fn longest_cycle(edges: Vec<i32>) -> i32 {
    let len = edges.len();
    let mut indegree = vec![0; len];
    for &edge in &edges {
        if edge != -1 {
            indegree[edge as usize] += 1;
        }
    }
    let mut seen = vec![false; len];
    let mut q = VecDeque::new();
    for i in 0..len {
        if indegree[i] == 0 {
            q.push_back(i);
            seen[i] = true;
        }
    }
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        if edges[u] != -1 {
            let v = edges[u] as usize;
            indegree[v] -= 1;
            if indegree[v] == 0 {
                q.push_back(v);
                seen[v] = true;
            }
        }
    }
    let mut result = -1;
    for i in 0..len {
        if !seen[i] {
            let mut cnt = 0;
            let mut cur = i as i32;
            while cur != -1 && !seen[cur as usize] {
                seen[cur as usize] = true;
                cur = edges[cur as usize];
                cnt += 1;
            }
            result = result.max(cnt);
        }
    }
    result
}

/// 时间戳
pub fn longest_cycle2(edges: Vec<i32>) -> i32 {
    let len = edges.len();
    let mut time = vec![0; len];
    let mut clock = 1;
    let mut ans = -1;
    for i in 0..len {
        if time[i] > 0 {
            continue;
        }
        let mut x = i as i32;
        let start_time = clock;
        while x >= 0 {
            if time[x as usize] > 0 {
                if time[x as usize] >= start_time {
                    ans = ans.max(clock - time[x as usize]);
                }
                break;
            }
            time[x as usize] = clock;
            clock += 1;
            x = edges[x as usize];
        }
    }
    ans
}

fn main() {
    fn test(func: fn(edges: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 3, 4, 2, 3]), 3);
        assert_eq!(func(vec![2, -1, 3, 1]), -1);
    }
    test(longest_cycle);
    test(longest_cycle2);
}
