//! 参加会议的最多员工数

use std::collections::VecDeque;

/// 内向基环树, 拓扑排序
pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
    fn dfs(m: &Vec<Vec<usize>>, cur: usize, ring: &mut Vec<usize>, vis: &mut Vec<bool>) {
        vis[cur] = true;
        ring.push(cur);
        for &nxt in &m[cur] {
            if !vis[nxt] {
                dfs(m, nxt, ring, vis);
            }
        }
    }

    // 通过反图 rg 寻找树枝上最深的链
    fn rdfs(m: &Vec<Vec<usize>>, cur: usize, prev: usize, cur_depth: i32, max_depth: &mut i32) {
        *max_depth = (*max_depth).max(cur_depth);
        for &nxt in &m[cur] {
            if nxt != prev {
                rdfs(m, nxt, cur, cur_depth + 1, max_depth);
            }
        }
    }

    let len = favorite.len();
    let mut m = vec![vec![]; len];
    let mut rm = vec![vec![]; len];
    let mut ind = vec![0; len];
    for i in 0..len {
        m[i].push(favorite[i] as usize);
        rm[favorite[i] as usize].push(i);
        ind[favorite[i] as usize] += 1;
    }

    // 拓扑排序，减去所有树枝，只剩下环
    let mut q = VecDeque::new();
    for i in 0..len {
        if ind[i] == 0 {
            q.push_back(i);
        }
    }
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        for &nxt in &m[node] {
            ind[nxt] -= 1;
            if ind[nxt] == 0 {
                q.push_back(nxt);
            }
        }
    }

    let mut vis = vec![false; len];
    let mut huan_max = 0;
    let mut zhi_sum_max = 0;
    for i in 0..len {
        if ind[i] > 0 && !vis[i] {
            let mut ring = vec![];
            dfs(&m, i, &mut ring, &mut vis);
            if ring.len() == 2 {
                // 环长度为2，接下来找最长的树枝
                let mut depth = 0;
                rdfs(&rm, ring[0], ring[1], 1, &mut depth);
                zhi_sum_max += depth;
                depth = 0;
                rdfs(&rm, ring[1], ring[0], 1, &mut depth);
                zhi_sum_max += depth;
            } else {
                huan_max = huan_max.max(ring.len() as i32);
            }
        }
    }

    return huan_max.max(zhi_sum_max);
}

fn main() {
    assert_eq!(maximum_invitations(vec![1, 0, 3, 2, 5, 6, 7, 4, 9, 8, 11, 10, 11, 12, 10]), 11);
    assert_eq!(maximum_invitations(vec![2, 2, 1, 2]), 3);
    assert_eq!(maximum_invitations(vec![1, 2, 0]), 3);
    assert_eq!(maximum_invitations(vec![3, 0, 1, 4, 1]), 4);
}
