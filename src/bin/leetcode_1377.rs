//! T 秒后青蛙的位置

use std::collections::VecDeque;

pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
    let n = n as usize;
    let mut g = vec![vec![]; n + 1];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    let mut q = VecDeque::new();
    q.push_back((1, 1., 0, 0));
    while !q.is_empty() {
        let (u, p, parent, second) = q.pop_front().unwrap();
        let children_len = g[u].len() - if parent == 0 { 0 } else { 1 };
        if u == target as usize {
            if second == t || children_len == 0 {
                return p;
            }
            break;
        }
        for &v in &g[u] {
            if v != parent && second + 1 <= t {
                q.push_back((v, p / children_len as f64, u, second + 1));
            }
        }
    }
    0.
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64) {
        assert_eq!(func(7,
                        vec![vec![1,2],vec![1,3],vec![1,7],vec![2,4],vec![2,6],vec![3,5]],
                        20,
                        6), 0.16666666666666666);
        assert_eq!(func(7, vec![vec![1, 2], vec![1, 3], vec![1, 7], vec![2, 4], vec![2, 6], vec![3, 5]], 2, 4), 0.16666666666666666);
        assert_eq!(func(7, vec![vec![1, 2], vec![1, 3], vec![1, 7], vec![2, 4], vec![2, 6], vec![3, 5]], 1, 7), 0.3333333333333333);
    }
    test(frog_position);
}
