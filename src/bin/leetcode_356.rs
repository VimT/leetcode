//! 直线镜像

use std::collections::HashMap;

pub fn is_reflected(points: Vec<Vec<i32>>) -> bool {
    let mut map = HashMap::new();
    for point in points {
        map.entry(point[1]).or_insert(vec![]).push(point[0]);
    }
    let mut mid = None;
    for (_, mut x) in map {
        x.sort_unstable();
        // 去重
        x.dedup();
        let mut y0 = None;
        let (mut i, mut j) = (0, x.len());
        while i < j {
            let cur = (x[j - 1] - x[i]) as f64 / 2.0 + x[i] as f64;
            if let Some(y) = y0 {
                if y as f64 != cur {
                    return false;
                }
            } else {
                y0 = Some(cur);
            }
            i += 1;
            j -= 1;
        }

        if let (Some(y0), Some(y1)) = (mid, y0) {
            if y0 as f64 != y1 {
                return false;
            }
        } else {
            mid = y0;
        }
    }
    true
}

fn main() {
    fn test(func: fn(points: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(vec![vec![0, 0], vec![1, 0]]), true);
        assert_eq!(func(vec![vec![0, 0], vec![1, 0], vec![3, 0]]), false);
        assert_eq!(func(vec![vec![-16, 1], vec![16, 1], vec![16, 1]]), true);
        assert_eq!(func(vec![vec![0, 0]]), true);
        assert_eq!(func(vec![vec![1, 1], vec![-1, 1]]), true);
        assert_eq!(func(vec![vec![1, 1], vec![-1, -1]]), false);
    }
    test(is_reflected);
}
