//! 最小面积矩形


use std::collections::HashSet;

pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
    let mut result = i32::MAX;
    let mut mem = HashSet::new();
    for point in points {
        let (x1, y1) = (point[0], point[1]);
        for &(x2, y2) in &mem {
            if mem.contains(&(x1, y2)) && mem.contains(&(x2, y1)) {
                result = result.min((x2 - x1).abs() * (y2 - y1).abs());
            }
        }
        mem.insert((x1, y1));
    }
    if result == i32::MAX { 0 } else { result }
}

fn main() {
    assert_eq!(min_area_rect(vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3], vec![2, 2]]), 4);
    assert_eq!(min_area_rect(vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3], vec![4, 1], vec![4, 3]]), 2);
}
