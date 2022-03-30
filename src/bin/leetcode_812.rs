//! 最大三角形面积

/// 鞋带公式
pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let len = points.len();
    let mut result: f64 = 0.;
    for i in 0..len {
        for j in i + 1..len {
            for k in j + 1..len {
                result = result.max(
                    0.5 * (points[i][0] * points[j][1]
                        + points[j][0] * points[k][1]
                        + points[k][0] * points[i][1]
                        - points[i][1] * points[j][0]
                        - points[j][1] * points[k][0]
                        - points[k][1] * points[i][0]).abs() as f64
                )
            }
        }
    }
    result
}

fn main() {
    assert_eq!(largest_triangle_area(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]]), 2.00000);
    assert_eq!(largest_triangle_area(vec![vec![1, 0], vec![0, 0], vec![0, 1]]), 0.50000);
}
