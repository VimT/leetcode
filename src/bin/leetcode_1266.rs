//! 访问所有点的最小时间

pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for point in points.windows(2) {
        let dx = (point[0][0] - point[1][0]).abs();
        let dy = (point[0][1] - point[1][1]).abs();
        result += dx.min(dy) + (dx - dy).abs();

    }
    result
}

fn main() {
    fn test(func: fn(points: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]), 7);
        assert_eq!(func(vec![vec![3, 2], vec![-2, 2]]), 5);
    }
    test(min_time_to_visit_all_points);
}
