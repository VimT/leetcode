//! 项目管理

pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
    let m = m as usize;
    let n = n as usize;
    let mut group_edge = vec![vec![]; m + 1];
    let mut group_in_degree = vec![0; m + 1];
    let mut groups = vec![vec![]; m + 1];

    let mut edge = vec![vec![]; n];
    let mut in_degree = vec![0; n];
    for (idx, items) in before_items.iter().enumerate() {
        in_degree[idx] = items.len();
        let group_to = (group[idx] + 1) as usize;
        groups[group_to].push(idx);
        for &j in items {
            edge[j as usize].push(idx);
            let group_from = (group[j as usize] + 1) as usize;
            if group_from != group_to {
                group_edge[group_from].push(group_to);
                if group_to != 0 {
                    group_in_degree[group_to] += 1;
                }
            }
        }
    }

    let mut gl = vec![];
    let mut s: Vec<usize> = group_in_degree.iter().enumerate().filter(|(_, y)| **y == 0).map(|(x, _)| x).collect();
    while !s.is_empty() {
        let group_node = s.pop().unwrap();
        gl.push(group_node);
        for &i in &group_edge[group_node] {
            group_in_degree[i] -= 1;
            if group_in_degree[i] == 0 {
                s.push(i);
            }
        }
    }
    for v in group_in_degree {
        if v > 0 {
            return vec![];
        }
    }
    let mut ans = vec![];
    let mut visited = vec![false; n];
    for group_idx in gl {
        let mut s = vec![];
        for idx in &groups[group_idx] {
            if in_degree[*idx] == 0 {
                s.push(*idx);
            }
        }
        while !s.is_empty() {
            let node = s.pop().unwrap();
            if visited[node] {
                continue;
            }
            ans.push(node as i32);
            visited[node] = true;
            for &i in &edge[node] {
                in_degree[i] -= 1;
                if in_degree[i] == 0 && (group[node] == group[i] || group[i] == -1) {
                    s.push(i);
                }
            }
        }

        if group_idx > 0 {
            for idx in &groups[group_idx] {
                if in_degree[*idx] > 0 {
                    return vec![];
                }
            }
        }
    }
    ans
}


fn main() {
    assert_eq!(sort_items(8, 2, vec![-1, -1, 1, 0, 0, 1, 0, -1], vec![vec![], vec![6], vec![5], vec![6], vec![3, 6], vec![], vec![], vec![]]), vec![5, 2, 6, 3, 4, 1, 7, 0]);
    assert_eq!(sort_items(8, 2, vec![-1, -1, 1, 0, 0, 1, 0, -1], vec![vec![], vec![6], vec![5], vec![6], vec![3], vec![], vec![4], vec![]]), vec![]);
}
