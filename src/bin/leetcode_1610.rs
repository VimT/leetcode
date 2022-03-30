//! 可见点的最大数目

use std::f64::consts::PI;

pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
    let mut same_cnt = 0;
    let mut polar_degrees = vec![];
    for point in &points {
        if point == &location {
            same_cnt += 1;
        } else {
            // 转换为极角
            polar_degrees.push(((point[1] - location[1]) as f64).atan2((point[0] - location[0]) as f64));
        }
    }
    polar_degrees.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let len = polar_degrees.len();
    // 由于存在 d + angel 180° 的情况，可以在原数组中将每个元素 +360° 添加到原数组的后面，可防止反转
    let mut new_polar_degrees: Vec<f64> = polar_degrees.iter().map(|x| *x + 2. * PI).collect();
    polar_degrees.append(&mut new_polar_degrees);
    let mut max_cnt = 0;
    let mut right = 0;
    let degree = angle as f64 * PI / 180.0;
    for i in 0..len {
        while right < len * 2 && polar_degrees[right] <= polar_degrees[i] + degree {
            right += 1;
        }
        max_cnt = max_cnt.max((right - i) as i32);
    }
    same_cnt + max_cnt
}

fn main() {
    assert_eq!(visible_points(vec![vec![2, 1], vec![2, 2], vec![3, 3]], 90, vec![1, 1]), 3);
    assert_eq!(visible_points(vec![vec![2, 1], vec![2, 2], vec![3, 4], vec![1, 1]], 90, vec![1, 1]), 4);
    assert_eq!(visible_points(vec![vec![1, 0], vec![2, 1]], 13, vec![1, 1]), 1);
}
