//! 课程表

use std::collections::LinkedList;

/// 被当前节点启动 flags[i] = 1， 被其他节点启动： flags[i] = -1;
pub fn can_finish_dfs(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut adjacency = vec![vec![]; num_courses as usize];
    let mut flags = vec![0; num_courses as usize];

    for i in prerequisites {
        adjacency[i[1] as usize].push(i[0] as usize);
    }
    fn dfs(i: usize, adjacency: &Vec<Vec<usize>>, flags: &mut Vec<i32>) -> bool {
        let i = i as usize;
        if flags[i] == -1 { return true; }
        if flags[i] == 1 { return false; }
        flags[i] = 1;
        for &j in &adjacency[i] {
            if !dfs(j, adjacency, flags) { return false; }
        }
        flags[i] = -1;
        true
    }
    for i in 0..num_courses as usize {
        if !dfs(i, &adjacency, &mut flags) {
            return false;
        }
    }
    true
}

/// 判断是否是有向无环图
/// 拓扑排序
pub fn can_finish_bfs(mut num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    // 邻接表
    let mut adjacency: Vec<Vec<usize>> = vec![vec![]; num_courses as usize];
    // 入度表
    let mut in_degree: Vec<usize> = vec![0; num_courses as usize];
    for i in prerequisites {
        adjacency[i[1] as usize].push(i[0] as usize);
        in_degree[i[0] as usize] += 1;
    }
    let mut queue: LinkedList<usize> = in_degree.iter().enumerate().filter(|(_, degree)| **degree == 0).map(|(node, _)| node).collect();
    while !queue.is_empty() {
        let start_node = queue.pop_front().unwrap();
        // 删除此节点==所有邻接节点 入度-1，
        for &n in &adjacency[start_node] {
            in_degree[n] -= 1;
            if in_degree[n] == 0 {
                queue.push_back(n);
            }
        }
        num_courses -= 1;
    }

    // 所有节点都入队出队过，说明无环
    return num_courses == 0;
}

fn main() {
    fn test(func: fn(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(2, vec![vec![1, 0]]), true);
        assert_eq!(func(2, vec![vec![1, 0], vec![0, 1]]), false);
    }
    test(can_finish_bfs);
    test(can_finish_dfs);
}
