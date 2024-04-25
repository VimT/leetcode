//! 覆盖所有点的最少矩形数目

use std::collections::BTreeSet;

pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
    let mut xs = BTreeSet::new();
    for point in points {
        xs.insert(point[0] as i64);
    }
    let mut last = i32::MIN as i64;
    let mut result = 0;
    for x in xs {
        if x - last > w as i64 {
            result += 1;
            last = x;
        }
    }
    result
}

fn main() {
    fn test(func: fn(points: Vec<Vec<i32>>, w: i32) -> i32) {
        assert_eq!(func(vec![vec![2, 1], vec![1, 0], vec![1, 4], vec![1, 8], vec![3, 5], vec![4, 6]], 1), 2);
        assert_eq!(func(vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6]], 2), 3);
        assert_eq!(func(vec![vec![2, 3], vec![1, 2]], 0), 2);
    }
    test(min_rectangles_to_cover_points);
}
