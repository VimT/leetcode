//! 镜面反射

/// 反射定律
pub fn mirror_reflection(p: i32, q: i32) -> i32 {
    let p = p as f64;
    let q = q as f64;
    let mut x = 0.0;
    let mut y = 0.0;
    let mut rx = p as f64;
    let mut ry = q as f64;
    const EPS: f64 = 1e-6;
    #[inline]
    fn close(x: f64, y: f64) -> bool {
        (x - y).abs() < EPS
    }
    while !(close(x, p) && (close(y, 0.0) || close(y, p)) || close(x, 0.0) && close(y, p)) {
        let mut t = 1e9 as f64;
        if (-x / rx) > EPS { t = t.min(-x / rx); }
        if (-y / ry) > EPS { t = t.min(-y / ry); }
        if (p - x) / rx > EPS { t = t.min((p - x) / rx); }
        if (p - y) / ry > EPS { t = t.min((p - y) / ry); }
        x += rx * t;
        y += ry * t;
        if close(x, p) || close(x, 0.0) { rx *= -1.0; }
        if close(y, p) || close(y, 0.0) { ry *= -1.0; }
    }
    if close(x, p) && close(y, p) { return 1; }
    if close(x, p) { 0 } else { 2 }
}

fn main() {
    assert_eq!(mirror_reflection(2, 1), 2);
    assert_eq!(mirror_reflection(3, 1), 1);
}
