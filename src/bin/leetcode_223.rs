//! 矩形面积


/// https://leetcode-cn.com/problems/rectangle-area/solution/223-ju-xing-mian-ji-3xing-dai-ma-by-acw_weian/
pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
    let (a, b, c, d, e, f, g, h) = (a as i64, b as i64, c as i64, d as i64, e as i64, f as i64, g as i64, h as i64);
    let x = 0.max(g.min(c) - a.max(e));
    let y = 0.max(d.min(h) - b.max(f));
    return ((c - a) * (d - b) - x * y + (g - e) * (h - f)) as i32;
}


fn main() {
    assert_eq!(compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    assert_eq!(compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
}
