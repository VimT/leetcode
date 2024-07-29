//! 判断矩形的两个角落是否可达

use leetcode::union_find::UnionFind;

/// 并查集
pub fn can_reach_corner(x: i32, y: i32, circles: Vec<Vec<i32>>) -> bool {
    let len = circles.len();
    // 并查集中 n 表示左边界和上边界，n+1 表示右边界和下边界
    let mut uf = UnionFind::new(len + 2);
    for (i, circle) in circles.iter().enumerate() {
        let (ox, oy, r) = (circle[0], circle[1], circle[2]);
        // 圆i和左边界或者上边界有交集
        if ox <= r || oy + r >= y { uf.union(i, len); }
        // 圆i和右边界或者下边界有交集
        if oy <= r || ox + r >= x { uf.union(i, len + 1); }
        for (j, c2) in circles[..i].iter().enumerate() {
            let (qx, qy, qr) = (c2[0], c2[1], c2[2]);
            if (ox - qx).pow(2) + (oy - qy).pow(2) <= (r + qr).pow(2) {
                uf.union(i, j); // 圆i和圆j有交集
            }
        }
        if uf.find(len) == uf.find(len + 1) { return false; }
    }
    true
}

fn main() {
    fn test(func: fn(x: i32, y: i32, circles: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(3, 4, vec![vec![2, 1, 1]]), true);
        assert_eq!(func(3, 3, vec![vec![1, 1, 2]]), false);
        assert_eq!(func(3, 3, vec![vec![2, 1, 1], vec![1, 2, 1]]), false);
    }
    test(can_reach_corner);
}
