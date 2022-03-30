//! 圆和矩形是否有重叠

pub fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    let x0 = (x1 + x2) as f64 / 2.0;
    let y0 = (y1 + y2) as f64 / 2.0;
    let p = ((x_center as f64 - x0).abs(), (y_center as f64 - y0).abs());
    let q = (x2 as f64 - x0, y2 as f64 - y0);
    let u = ((p.0 - q.0).max(0.0), (p.1 - q.1).max(0.0));
    return (u.0 * u.0 + u.1 * u.1).sqrt() <= radius as f64;
}


fn main() {
    assert_eq!(check_overlap(1, 0, 0, 1, -1, 3, 1), true);
    assert_eq!(check_overlap(1, 1, 1, 1, -3, 2, -1), false);
    assert_eq!(check_overlap(1, 0, 0, -1, 0, 0, 1), true);
}
