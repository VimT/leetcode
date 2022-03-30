//! 判断二分图

use std::collections::VecDeque;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Status {
    None,
    A,
    B,
}

pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let len = graph.len();
    let mut visited = vec![Status::None; len];
    for i in 0..len {
        if visited[i] == Status::None {
            let mut q = VecDeque::new();
            q.push_back(i);
            visited[i] = Status::A;
            while !q.is_empty() {
                let node = q.pop_front().unwrap();
                let expect = if visited[node] == Status::A { Status::B } else { Status::A };
                for i in &graph[node] {
                    let idx = *i as usize;
                    if visited[idx] == Status::None {
                        q.push_back(idx);
                        visited[idx] = expect;
                    } else if visited[idx] != expect {
                        return false;
                    }
                }
            }
        }
    }
    true
}
fn main() {
    assert_eq!(is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]]), false);
    assert_eq!(is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]]), true);
}
