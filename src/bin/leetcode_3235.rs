//! 判断矩形的两个角落是否可达

use leetcode::union_find::UnionFind;

pub fn can_reach_corner(x: i32, y: i32, circles: Vec<Vec<i32>>) -> bool {
    let x = x as i64;
    let y = y as i64;
    let n = circles.len();
    let mut uf = UnionFind::new(n + 2);

    // 计算两点之间距离的平方
    let dis = |p1: (i64, i64), p2: (i64, i64)| -> i64 {
        (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)
    };

    // 检查两条线段是否相交
    let check = |p: (i64, i64), q: (i64, i64), a: (i64, i64), b: (i64, i64)| -> bool {
        let c1 = (a.0 - p.0) * (q.1 - p.1) - (a.1 - p.1) * (q.0 - p.0);
        let c2 = (b.0 - p.0) * (q.1 - p.1) - (b.1 - p.1) * (q.0 - p.0);
        !((c1 > 0 && c2 > 0) || (c1 < 0 && c2 < 0))
    };

    for i in 0..n {
        let (x1, y1, r1) = (circles[i][0] as i64, circles[i][1] as i64, circles[i][2] as i64);
        let center1 = (x1, y1);

        // 检查圆是否与左边界或上边界相交
        if (y1 <= y && dis(center1, (0, y1)) <= r1 * r1) || (x1 <= x && dis(center1, (x1, y)) <= r1 * r1) {
            uf.union(i, n);
        }
        // 检查圆是否与右边界或下边界相交
        if (y1 <= y && dis(center1, (x, y1)) <= r1 * r1) || (x1 <= x && dis(center1, (x1, 0)) <= r1 * r1) {
            uf.union(i, n + 1);
        }

        // 检查圆与圆之间的相交情况
        for j in (i + 1)..n {
            let (x2, y2, r2) = (circles[j][0] as i64, circles[j][1] as i64, circles[j][2] as i64);
            let center2 = (x2, y2);
            let d = dis(center1, center2);
            if d > (r1 + r2).pow(2) {
                continue;  // 两圆不相交
            }

            // 如果任一圆在矩形内，则合并
            if (x1 >= 0 && x1 <= x && y1 >= 0 && y1 <= y) || (x2 >= 0 && x2 <= x && y2 >= 0 && y2 <= y) {
                uf.union(i, j);
                continue;
            }

            // 检查圆心连线是否与矩形对角线相交
            if check((0, 0), (x, y), center1, center2) && check(center1, center2, (0, 0), (x, y)) {
                uf.union(i, j);
                continue;
            }
            // 检查圆心连线是否与矩形另一对角线相交
            if check((0, y), (x, 0), center1, center2) && check(center1, center2, (0, y), (x, 0)) {
                uf.union(i, j);
                continue;
            }
        }
        if uf.find(n) == uf.find(n + 1) {
            return false;
        }
    }
    true
}

fn main() {
    fn test(func: fn(x: i32, y: i32, circles: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(3, 3, vec![vec![2, 1000, 997], vec![1000, 2, 997]]), true);
        assert_eq!(func(3, 4, vec![vec![2, 1, 1]]), true);
        assert_eq!(func(3, 3, vec![vec![1, 1, 2]]), false);
        assert_eq!(func(3, 3, vec![vec![2, 1, 1], vec![1, 2, 1]]), false);
    }
    test(can_reach_corner);
}
