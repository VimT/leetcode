//! 捕获黑皇后需要的最少移动次数

pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    // 检查白车是否可以捕获黑皇后
    // 同时检查白象是否挡在白车的路上
    if a == e && (c != a || d < b.min(f) || d > b.max(f)) {
        return 1;
    }
    if b == f && (c < a.min(e) || c > a.max(e) || d != b) {
        return 1;
    }

    // 检查白象是否可以捕获黑皇后
    // 考虑两个对角线方向
    if (c - d) == (e - f) && ((a - b) != (c - d) || a < c.min(e) || a > c.max(e)) {
        return 1;
    }
    if c + d == e + f && (a + b != c + d || a < c.min(e) || a > c.max(e)) {
        return 1;
    }
    return 2;
}

fn main() {
    fn test(func: fn(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32) {
        assert_eq!(func(7, 5, 2, 5, 6, 3), 2);
        assert_eq!(func(1, 1, 8, 8, 2, 3), 2);
        assert_eq!(func(5, 3, 3, 4, 5, 2), 1);
    }
    test(min_moves_to_capture_the_queen);
}
