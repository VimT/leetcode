//! 按距离统计房屋对数目 II

/// 计算每个房子i，从i到j的距离 (j > i)
pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i64> {
    if (x - y).abs() <= 1 {
        return (0..n as usize).map(|x| x as i64 * 2).rev().collect();
    }
    let mut x = x - 1;
    let mut y = y - 1;
    if x > y { std::mem::swap(&mut x, &mut y); }
    let mut diff = vec![0; n as usize];
    let mut add = |x: i32, y: i32| {
        if x > y { return; }
        diff[x as usize] += 2;
        if y + 1 < n { diff[y as usize + 1] -= 2; }
    };
    let perimeter = y - x + 1; // 环周长
    let mid = (x + y) / 2;
    let dis = y - x;
    let half = dis / 2;
    for i in 0..n {
        if i <= x {
            add(0, mid - i - 1);  // i->mid
            add(x - i, x - i + perimeter / 2 - 1);  // i->x->y->j (mid<j<y)
            add(x - i + 1, x - i + n - y - 1); // i->x->y->j (j>y)
        } else if i >= y {
            add(0, n - 2 - i);  // i->n
        } else {
            let to_x = i - x;
            let to_y = y - i;
            if to_x >= to_y - 1 {
                add(0, n - 2 - i); // i->n
            } else {
                add(0, half - 1); // i向后走半圈
                add(to_x, dis - half - 1); // i向前走半圈
                add(to_x + 1, to_x + n - y - 1); // i->x->y->n
            }
        }
    }

    for i in 1..n as usize {
        diff[i] += diff[i - 1];
    }
    diff
}

fn main() {
    fn test(func: fn(n: i32, x: i32, y: i32) -> Vec<i64>) {
        assert_eq!(func(6, 1, 6), vec![12, 12, 6, 0, 0, 0]);
        assert_eq!(func(3, 1, 3), vec![6, 0, 0]);
        assert_eq!(func(4, 1, 1), vec![6, 4, 2, 0]);
        assert_eq!(func(5, 2, 4), vec![10, 8, 2, 0, 0]);
    }
    test(count_of_pairs);
}
