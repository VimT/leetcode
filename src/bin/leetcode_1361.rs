//! 验证二叉树

use std::collections::VecDeque;
use leetcode::union_find::UnionFind;

/// 拓扑排序
pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    let len = n as usize;
    let mut indegree = vec![0; len];
    for i in 0..len {
        if left_child[i] != -1 { indegree[left_child[i] as usize] += 1; }
        if right_child[i] != -1 { indegree[right_child[i] as usize] += 1; }
    }
    let mut root = vec![];
    for i in 0..len {
        if indegree[i] == 0 {
            root.push(i);
        } else if indegree[i] > 1 {
            return false;
        }
    }
    if root.len() != 1 { return false; }
    let root = root[0];
    let mut q = VecDeque::new();
    q.push_back(root);
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        for child in [left_child[node], right_child[node]] {
            if child != -1 {
                indegree[child as usize] -= 1;
                if indegree[child as usize] == 0 {
                    q.push_back(child as usize);
                }
            }
        }
    }
    indegree.into_iter().all(|x| x == 0)
}

/// 并查集
pub fn validate_binary_tree_nodes2(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    let n = n as usize;
    let mut uf = UnionFind::new(n);
    let mut ind = vec![false; n];
    for ((i, l), r) in (0..n).zip(left_child).zip(right_child) {
        let pp = us.find(i);
        for child in [l, r] {
            if child != -1 {
                if ind[child as usize] { return false; }
                let ss = us.find(child as usize);
                if pp == ss {
                    return false;
                }
                us.union(pp, ss);
                ind[child as usize] = true;
            }
        }
    }
    us.count == 1
}

fn main() {
    fn test(func: fn(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool) {
        assert_eq!(func(3, vec![1, -1, -1], vec![-1, -1, 1]), false);
        assert_eq!(func(4, vec![1, -1, 3, -1], vec![2, -1, -1, -1]), true);
        assert_eq!(func(4, vec![1, -1, 3, -1], vec![2, 3, -1, -1]), false);
        assert_eq!(func(2, vec![1, 0], vec![-1, -1]), false);
    }
    test(validate_binary_tree_nodes);
    test(validate_binary_tree_nodes2);
}
