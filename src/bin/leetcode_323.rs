//! 无向图中连通分量的数目


use leetcode::union_set::UnionSet;

pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut us = UnionSet::new(n as usize);
    for edge in edges {
        us.union(edge[0] as usize, edge[1] as usize);
    }
    us.count as i32
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]), 2);
        assert_eq!(func(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]), 1);
    }
    test(count_components);
}
