//! 判断能否在给定时间到达单元格

pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
    if (sx, sy) == (fx, fy) && t == 1 { return false; }
    let dx = (sx as i64 - fx as i64).abs();
    let dy = (sy as i64 - fy as i64).abs();
    return dx <= t as i64 && dy <= t as i64;
}

fn main() {
    fn test(func: fn(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool) {
        assert_eq!(func(2, 4, 7, 7, 6), true);
        assert_eq!(func(3, 1, 7, 3, 3), false);
    }
    test(is_reachable_at_time);
}
