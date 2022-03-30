//! 直线上最多的点数

use std::collections::HashSet;
use std::mem::swap;

pub fn gcd(mut a: i32, mut b: i32) {
    if a < b { swap(&mut a, &mut b); }
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
}

pub fn resolve(p1: (i32, i32), p2: (i32, i32)) -> (f64, f64) {
    let a = (p2.1 - p1.1) as f64 / (p2.0 - p1.0) as f64;
    if a.is_infinite() { return (a.abs(), p1.0 as f64); }
    let b = p1.1 as f64 - a * p1.0 as f64;
    return (a, b);
}

/// 两点式验证
pub fn test(p1: (i32, i32), p2: (i32, i32), p: (i32, i32)) -> bool {
    return (p2.1 - p1.1) as i64 * (p.0 - p2.0) as i64 == (p.1 - p2.1) as i64 * (p2.0 - p1.0) as i64;
}

/// 斜截式
pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    if points.len() <= 2 { return points.len() as i32; }
    let mut set: HashSet<String> = HashSet::new();
    let points: Vec<(i32, i32)> = points.into_iter().map(|x| (x[0], x[1])).collect();
    let mut i = 1;
    // 判断所有点都相等的情况
    while i < points.len() {
        if points[i] != points[i - 1] {
            break;
        }
        i += 1;
    }
    if i == points.len() {
        return i as i32;
    }
    let mut ans = 0;
    for i1 in 0..points.len() - 1 {
        for i2 in i1 + 1..points.len() {
            if points[i1] == points[i2] { continue; }
            let key = resolve(points[i1], points[i2]);
            let key = format!("{}@{}", key.0, key.1);
            if set.contains(&key) { continue; }

            let mut count = 2;
            for i in 0..points.len() {
                if i != i1 && i != i2 && test(points[i1], points[i2], points[i]) { count += 1; }
            }
            // println!("{:?},{:?},{},{}", points[i1], points[i2], key, count);

            ans = ans.max(count);
            set.insert(key);
        }
    }
    ans
}

/// 判断经过此点的斜率，斜率相同说明在同一条直线上
pub fn max_points_b(points: Vec<Vec<i32>>) -> i32 { 0 }

/// 别人的，排序后，只需要和前一个点组成线？
pub fn max_points_best(points: Vec<Vec<i32>>) -> i32 {
    let len = points.len();
    if len <= 2 {
        return len as i32;
    }
    let mut points = points;
    points.sort_by(|a, b| {
        let da = (a[0] - a[1]).abs();
        let db = (b[0] - b[1]).abs();
        da.cmp(&db)
    });
    let mut max = 1;
    for i in 1..len {
        let x = points[i][0];
        let y = points[i][1];
        let dx = points[i - 1][0] - x;
        let dy = points[i - 1][1] - y;

        let mut count = 0;
        if dx == 0 && dy == 0 {
            for j in 0..len {
                if points[j][0] == x && points[j][1] == y {
                    count += 1;
                }
            }
        } else {
            for j in 0..len {
                if (points[j][0] - x) as i64 * dy as i64 == (points[j][1] - y) as i64 * dx as i64 {
                    count += 1;
                }
            }
        }

        if count > max {
            max = count;
        }
    }
    max as i32
}

fn main() {
    fn test(func: fn(points: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 3);
        assert_eq!(func(vec![vec![1, 1], vec![3, 2], vec![5, 3], vec![4, 1], vec![2, 3], vec![1, 4]]), 4);
    }
    test(max_points);
    // test(max_points_b);
    test(max_points_best);
}
