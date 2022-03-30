//! 课程表 II

use std::collections::VecDeque;

/// 拓扑排序
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    // 邻接表
    let mut adjacency: Vec<Vec<usize>> = vec![vec![]; num_courses as usize];
    // 入度表
    let mut in_degree: Vec<usize> = vec![0; num_courses as usize];
    for i in prerequisites {
        adjacency[i[1] as usize].push(i[0] as usize);
        in_degree[i[0] as usize] += 1;
    }
    let mut queue: VecDeque<usize> = in_degree.iter().enumerate().filter(|(_, degree)| **degree == 0).map(|(node, _)| node).collect();
    let mut ans = vec![];

    while !queue.is_empty() {
        let start_node = queue.pop_front().unwrap();
        ans.push(start_node as i32);
        // 删除此节点==所有邻接节点 入度-1，
        for &n in &adjacency[start_node] {
            in_degree[n] -= 1;
            if in_degree[n] == 0 {
                queue.push_back(n);
            }
        }
    }

    // 所有节点都入队出队过，说明无环
    return if ans.len() == num_courses as usize { ans } else { vec![] };
}

/// dfs到达所有叶子节点，存入栈中
pub fn find_order_dfs(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut adjacency = vec![vec![]; num_courses as usize];
    let mut flags = vec![0; num_courses as usize];

    for i in prerequisites {
        adjacency[i[1] as usize].push(i[0] as usize);
    }
    fn dfs(i: usize, adjacency: &Vec<Vec<usize>>, flags: &mut Vec<i32>, stack: &mut Vec<i32>) -> bool {
        let i = i as usize;
        if flags[i] == -1 { return true; }
        if flags[i] == 1 { return false; }
        flags[i] = 1;
        for &j in &adjacency[i] {
            if !dfs(j, adjacency, flags, stack) { return false; }
        }
        stack.push(i as i32);
        flags[i] = -1;
        true
    }
    let mut stack = vec![];
    for i in 0..num_courses as usize {
        if !dfs(i, &adjacency, &mut flags, &mut stack) {
            return vec![];
        }
    }
    stack.reverse();
    return stack;
}


fn main() {
    fn test(func: fn(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(2, vec![vec![1, 0]]), vec![0, 1]);
        assert_eq!(func(1, vec![]), vec![0]);
    }
    test(find_order);
    test(find_order_dfs);
}
