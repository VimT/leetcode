//! 情侣牵手


use leetcode::union_find::UnionFind;

pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
    let mut ans = 0;
    let len = row.len();
    for i in (0..len).step_by(2) {
        let b = row[i] ^ 1;
        if b == row[i + 1] { continue; }
        for j in 0..len {
            if row[j] == b {
                row.swap(j, i + 1);
                break;
            }
        }
        ans += 1;
    }
    ans
}


/// 至少交换的次数 = 所有情侣的对数 - 并查集里连通分量的个数
pub fn min_swaps_couples_us(row: Vec<i32>) -> i32 {
    let len = row.len();
    let n = len / 2;
    let mut uf = UnionFind::new(n);
    for i in (0..len).step_by(2) {
        us.union((row[i] / 2) as usize, (row[i + 1] / 2) as usize);
    }
    (n - us.count) as i32
}


fn main() {
    fn test(func: fn(row: Vec<i32>) -> i32) {
        assert_eq!(func(vec![0, 2, 1, 3]), 1);
        assert_eq!(func(vec![3, 2, 0, 1]), 0);
    }
    test(min_swaps_couples);
    test(min_swaps_couples_us);
}
