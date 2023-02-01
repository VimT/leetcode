//! 连通网络的操作次数

use leetcode::union_set::UnionSet;

pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let len = connections.len();
    let n = n as usize;
    if len + 1 < n {
        return -1;
    }
    let mut us = UnionSet::new(n);
    for connection in connections {
        us.union(connection[0] as usize, connection[1] as usize);
    }
    (us.count - 1) as i32
}

fn main() {
    assert_eq!(make_connected(5, vec![vec![0, 1], vec![0, 2], vec![3, 4], vec![2, 3]]), 0);
    assert_eq!(make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]), 1);
    assert_eq!(make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]]), 2);
    assert_eq!(make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]), -1);
}
