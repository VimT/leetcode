//! 带阈值的图连通性

use leetcode::union_find::UnionFind;

pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut uf = UnionFind::new(n as usize + 1);
    // 枚举公因数
    for z in threshold + 1..=n / 2 {
        let mut p = z;
        let mut q = z * 2;
        while q <= n {
            uf.union(p as usize, q as usize);
            p += z;
            q += z;
        }
    }
    queries.into_iter().map(|x| uf.find(x[0] as usize) == uf.find(x[1] as usize)).collect()
}

/// 埃氏筛
pub fn are_connected2(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut uf = UnionFind::new(n as usize + 1);
    let mut is_prime = vec![true; n as usize + 1];
    for z in threshold as usize + 1..=n as usize {
        if is_prime[z] {
            let mut p = z;
            let mut q = z * 2;
            while q <= n as usize {
                is_prime[q] = false;
                uf.union(p, q);
                p += z;
                q += z;
            }
        }
    }
    queries.into_iter().map(|x| uf.find(x[0] as usize) == uf.find(x[1] as usize)).collect()
}

fn main() {
    fn test(func: fn(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool>) {
        assert_eq!(func(14, 4, vec![vec![4, 2], vec![7, 2], vec![4, 3], vec![1, 4], vec![4, 11], vec![6, 8], vec![8, 12], vec![12, 5], vec![3, 7], vec![12, 6], vec![3, 6], vec![11, 9], vec![6, 9], vec![6, 4], vec![4, 9], vec![14, 4], vec![10, 14], vec![14, 2], vec![9, 8], vec![8, 7], vec![13, 14], vec![12, 4], vec![7, 4], vec![10, 4], vec![1, 6], vec![9, 7], vec![5, 13], vec![10, 11], vec![14, 8], vec![5, 6], vec![7, 12], vec![11, 5], vec![8, 13], vec![4, 8], vec![1, 9], vec![8, 2], vec![1, 13], vec![5, 9], vec![12, 1], vec![13, 10], vec![1, 8], vec![10, 6], vec![9, 13], vec![6, 11], vec![3, 5], vec![5, 2]]), vec![false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]);
        assert_eq!(func(6, 2, vec![vec![1, 4], vec![2, 5], vec![3, 6]]), vec![false, false, true]);
        assert_eq!(func(6, 0, vec![vec![4, 5], vec![3, 4], vec![3, 2], vec![2, 6], vec![1, 3]]), vec![true, true, true, true, true]);
        assert_eq!(func(5, 1, vec![vec![4, 5], vec![4, 5], vec![3, 2], vec![2, 3], vec![3, 4]]), vec![false, false, false, false, false]);
    }
    test(are_connected);
    test(are_connected2);
}
