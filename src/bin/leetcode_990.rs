//! 等式方程的可满足性

use std::collections::LinkedList;

pub fn equations_possible(equations: Vec<String>) -> bool {
    let mut eq_map = vec![vec![]; 26];
    let mut neq_map = vec![vec![]; 26];

    for i in equations.iter() {
        let s = i.as_bytes();
        if s[0] == s[3] {
            if s[1] == b'!' { return false; }
            continue;
        }
        if s[1] == b'=' {
            eq_map[(s[0] - b'a') as usize].push((s[3] - b'a') as usize);
            eq_map[(s[3] - b'a') as usize].push((s[0] - b'a') as usize);
        } else {
            neq_map[(s[0] - b'a') as usize].push((s[3] - b'a') as usize);
            neq_map[(s[3] - b'a') as usize].push((s[0] - b'a') as usize);
        }
    }
    let mut all_visited = vec![false; 26];
    for i in 0..26 {
        if eq_map[i].len() == 0 || all_visited[i] { continue; }
        let mut visited = vec![false; 26];
        let mut queue = LinkedList::new();
        queue.push_back(i);
        while !queue.is_empty() {
            let n = queue.pop_front().unwrap();
            visited[n] = true;
            all_visited[n] = true;
            for &i in &eq_map[n] {
                if !visited[i] {
                    queue.push_back(i);
                }
            }
        }
        for i in 0..26 {
            if visited[i] {
                for j in &neq_map[i] {
                    if visited[*j] { return false; }
                }
            }
        }
    }

    true
}

/// 使用parent数据结构，每个元素表示当前变量所在的连通分量的父节点信息，如果父节点是自身，说明该变量为所在的连通分量的根节点
pub fn equations_possible_parent(equations: Vec<String>) -> bool {
    let mut parent = (0..26).collect::<Vec<usize>>();
    fn find(parent: &mut Vec<usize>, mut i: usize) -> usize {
        while parent[i] != i {
            parent[i] = parent[parent[i]];
            i = parent[i]
        }
        i
    }
    for i in equations.iter() {
        let s = i.as_bytes();
        if s[1] == b'=' {
            let a = (s[0] - b'a') as usize;
            let b = (s[3] - b'a') as usize;
            let fa = find(&mut parent, a);
            let fb = find(&mut parent, b);
            parent[fa] = fb;
        }
    }
    for i in equations.iter() {
        let s = i.as_bytes();
        if s[1] == b'!' {
            let a = (s[0] - b'a') as usize;
            let b = (s[3] - b'a') as usize;
            if find(&mut parent, a) == find(&mut parent, b) {
                return false;
            }
        }
    }

    true
}


fn main() {
    assert_eq!(equations_possible_parent(vec!["a==b", "b!=a"].into_iter().map(|x| x.to_string()).collect()), false);
    assert_eq!(equations_possible_parent(vec!["a==b", "b==a"].into_iter().map(|x| x.to_string()).collect()), true);
}