//! 有效的回旋镖

pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
    points[0] != points[1] && points[1] != points[2] && points[0] != points[2] &&
        (points[2][0] - points[0][0]) * (points[1][1] - points[0][1]) != (points[1][0] - points[0][0]) * (points[2][1] - points[0][1])
}

fn main() {
    assert_eq!(is_boomerang(vec![vec![0, 0], vec![0, 2], vec![2, 1]]), true);
    assert_eq!(is_boomerang(vec![vec![1, 1], vec![2, 3], vec![3, 2]]), true);
    assert_eq!(is_boomerang(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), false);
}
