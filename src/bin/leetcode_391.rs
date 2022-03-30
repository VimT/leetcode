//! 完美矩形


use std::collections::HashMap;

pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, 0, 0);
    let mut area = 0;
    let mut mp: HashMap<(i32, i32), i32> = HashMap::new();
    for rect in rectangles.iter() {
        let (x, y, a, b) = (rect[0], rect[1], rect[2], rect[3]);
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(a);
        max_y = max_y.max(b);
        area += (a - x) * (b - y);
        *mp.entry((x, y)).or_default() += 1;
        *mp.entry((x, b)).or_default() += 1;
        *mp.entry((a, y)).or_default() += 1;
        *mp.entry((a, b)).or_default() += 1;
    }
    if (max_x - min_x) * (max_y - min_y) != area {
        return false;
    }
    for (k, v) in mp {
        if k == (min_x, min_y) || k == (max_x, min_y) || k == (min_x, max_y) || k == (max_x, max_y) {
            if v != 1 { return false; }
        } else {
            if v != 2 && v != 4 { return false; }
        }
    }
    true
}

fn main() {
    assert_eq!(is_rectangle_cover(vec![vec![1, 1, 3, 3], vec![3, 1, 4, 2], vec![3, 2, 4, 4], vec![1, 3, 2, 4], vec![2, 3, 3, 4]]), true);
    assert_eq!(is_rectangle_cover(vec![vec![1, 1, 2, 3], vec![1, 3, 2, 4], vec![3, 1, 4, 2], vec![3, 2, 4, 4]]), false);
    assert_eq!(is_rectangle_cover(vec![vec![1, 1, 3, 3], vec![3, 1, 4, 2], vec![1, 3, 2, 4], vec![3, 2, 4, 4]]), false);
    assert_eq!(is_rectangle_cover(vec![vec![1, 1, 3, 3], vec![3, 1, 4, 2], vec![1, 3, 2, 4], vec![2, 2, 4, 4]]), false);
}
