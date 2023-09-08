//! 保证图可完全遍历

use leetcode::union_find::UnionFind;

pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
    edges.sort_unstable_by_key(|x| -x[0]);

    let mut us1 = UnionFind::new(n as usize);
    let mut us2 = UnionFind::new(n as usize);
    let mut result = 0;
    for edge in edges {
        let (t, a, b) = (edge[0], edge[1] as usize - 1, edge[2] as usize - 1);
        match t {
            1 => result += 1 - us1.union(a, b) as i32,
            2 => result += 1 - us2.union(a, b) as i32,
            3 => result += 1 - (us1.union(a, b) | us2.union(a, b)) as i32,
            _ => ()
        }
    }
    if us1.count != 1 || us2.count != 1 { return -1; }
    result
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(4, vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 3], vec![1, 2, 4], vec![1, 1, 2], vec![2, 3, 4]]), 2);
        assert_eq!(func(4, vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]]), 0);
        assert_eq!(func(4, vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]]), -1);
    }
    test(max_num_edges_to_remove);
}
