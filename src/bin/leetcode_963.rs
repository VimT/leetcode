//! 最小面积矩形 II

use std::collections::HashSet;

fn dis(p1: (i32, i32), p2: (i32, i32)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    ((dx * dx) as f64 + (dy * dy) as f64).sqrt()
}

pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
    let len = points.len();
    let mut set = HashSet::new();
    for point in &points {
        set.insert((point[0], point[1]));
    }
    let mut result = f64::MAX;
    for i in 0..len {
        let p1 = (points[i][0], points[i][1]);
        for j in 0..len {
            if i == j { continue; }
            let p2 = (points[j][0], points[j][1]);
            for k in j + 1..len {
                if k == i { continue; }
                let p3 = (points[k][0], points[k][1]);
                let p4 = (p2.0 + p3.0 - p1.0, p2.1 + p3.1 - p1.1);
                if set.contains(&p4) {
                    let dot = (p2.0 - p1.0) * (p3.0 - p1.0) + (p2.1 - p1.1) * (p3.1 - p1.1);
                    if dot == 0 {
                        let area = dis(p1, p2) * dis(p1, p3);
                        result = result.min(area);
                    }
                }
            }
        }
    }
    if result == f64::MAX { 0. } else { result }
}

fn main() {
    assert!(min_area_free_rect(vec![vec![0, 3], vec![1, 2], vec![3, 1], vec![1, 3], vec![2, 1]]).abs() < 1e-5);
    assert!((min_area_free_rect(vec![vec![1, 2], vec![2, 1], vec![1, 0], vec![0, 1]]) - 2.00000).abs() < 1e-5);
    assert!((min_area_free_rect(vec![vec![0, 1], vec![2, 1], vec![1, 1], vec![1, 0], vec![2, 0]]) - 1.00000).abs() < 1e-5);
}
