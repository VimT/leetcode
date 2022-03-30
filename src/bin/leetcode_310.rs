//! 最小高度树


use std::collections::HashMap;

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut edge_map = vec![vec![]; n];
    let mut degree = vec![0; n];
    for edge in &edges {
        let i1 = edge[0] as usize;
        let i2 = edge[1] as usize;
        degree[i1] += 1;
        degree[i2] += 1;
        edge_map[i1].push(i2);
        edge_map[i2].push(i1);
    }

    fn dfs(h: &mut HashMap<(usize, usize), u32>, em: &Vec<Vec<usize>>, cur: usize, visited: &mut Vec<bool>) -> u32 {
        visited[cur] = true;
        let mut result = 0;
        for &node in &em[cur] {
            if !visited[node] {
                if let Some(v) = h.get(&(cur, node)) {
                    result = result.max(*v);
                    continue;
                }
                let height = dfs(h, em, node, visited) + 1;
                h.insert((cur, node), height);
                result = result.max(height);
            }
        }
        visited[cur] = false;
        result
    }
    let mut h = HashMap::new();
    let mut visited = vec![false; n];
    let mut nh = vec![0; n];
    for i in 0..n {
        dfs(&mut h, &edge_map, i, &mut visited);
        for &j in &edge_map[i] {
            nh[i] = nh[i].max(*h.get(&(i, j)).unwrap());
        }
    }
    let min = *nh.iter().min().unwrap();
    nh.into_iter().enumerate().filter(|(_, v)| *v == min).map(|(k, _)| k as i32).collect()
}

/// 拓扑排序
pub fn find_min_height_trees_tp(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    if n == 1 {
        return vec![0];
    }
    let mut edge_map = vec![vec![]; n];
    let mut degree = vec![0; n];
    for edge in &edges {
        let i1 = edge[0] as usize;
        let i2 = edge[1] as usize;
        degree[i1] += 1;
        degree[i2] += 1;
        edge_map[i1].push(i2);
        edge_map[i2].push(i1);
    }
    let mut queue = vec![];
    for i in 0..n {
        if degree[i] == 1 {
            queue.push(i);
        }
    }
    loop {
        let mut new_queue = vec![];
        for &i in &queue {
            for &j in &edge_map[i] {
                if degree[j] > 0 {
                    degree[j] -= 1;
                    if degree[j] == 1 {
                        new_queue.push(j);
                    }
                }
            }
        }
        if new_queue.is_empty() {
            return queue.iter().map(|x| *x as i32).collect();
        }
        queue = new_queue;
    }
}


fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]), vec![1]);
        assert_eq!(func(6, vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]), vec![3, 4]);
    }
    test(find_min_height_trees);
    test(find_min_height_trees_tp);
}
