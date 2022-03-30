//! 可能的二分法

use std::collections::VecDeque;

/// 这是广搜做法，并查集做法：对每条边，判断两边是否在同一棵树，是则return false；否则union(a, n+b) union(a+n, b), a+n表示a不喜欢的
pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    let mut edge = vec![vec![]; n + 1];
    for dislike in &dislikes {
        edge[dislike[0] as usize].push(dislike[1] as usize);
        edge[dislike[1] as usize].push(dislike[0] as usize);
    }
    let mut bet = vec![0; n + 1];
    for i in 1..=n {
        if bet[i] == 0 {
            bet[i] = 1;
            let mut q = VecDeque::new();
            q.push_back(i);
            while !q.is_empty() {
                let cur = q.pop_front().unwrap();
                for &nxt in &edge[cur] {
                    if bet[nxt] == 0 {
                        bet[nxt] = if bet[cur] == 1 { 2 } else { 1 };
                        q.push_back(nxt);
                    } else {
                        if bet[cur] == bet[nxt] {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

fn main() {
    assert_eq!(possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]), true);
    assert_eq!(possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]), false);
    assert_eq!(possible_bipartition(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]]), false);
}
