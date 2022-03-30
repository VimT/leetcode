//! 重构一棵树的方案数

use std::collections::{HashMap, HashSet};

pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
    let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
    for pair in &pairs {
        adj.entry(pair[0]).or_default().insert(pair[1]);
        adj.entry(pair[1]).or_default().insert(pair[0]);
    }
    let (root, root_neigh) = adj.iter().max_by_key(|x| x.1.len()).unwrap();
    if root_neigh.len() != adj.len() - 1 {
        return 0;
    }
    let mut result = 1;
    for (node, neighbour) in &adj {
        if node == root {
            continue;
        }
        let cur_degree = neighbour.len();
        let mut parent = -1;
        let mut parent_degree = usize::MAX;
        for &neighbour in neighbour {
            let nlen = adj[&neighbour].len();
            if cur_degree <= nlen && nlen < parent_degree {
                parent = neighbour;
                parent_degree = nlen;
            }
        }
        if parent == -1 {
            return 0;
        }
        // 检测 neighbours 是否是 adj[parent] 的子集
        for &neighbour in neighbour {
            if neighbour == parent {
                continue;
            }
            if !adj[&parent].contains(&neighbour) {
                return 0;
            }
        }
        if parent_degree == cur_degree {
            result = 2;
        }
    }
    result
}

fn main() {
    assert_eq!(check_ways(vec![vec![4, 5], vec![3, 4], vec![2, 4]]), 1);
    assert_eq!(check_ways(vec![vec![1, 2], vec![2, 3], vec![1, 3]]), 2);
    assert_eq!(check_ways(vec![vec![1, 2], vec![2, 3]]), 1);
    assert_eq!(check_ways(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![1, 5]]), 0);
}
