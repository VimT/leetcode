//! 使两个整数相等的位更改次数

pub fn min_changes(n: i32, k: i32) -> i32 {
    if n & k != k { return -1; }
    (n ^ k).count_ones() as i32
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> i32) {
        assert_eq!(func(13, 4), 2);
        assert_eq!(func(21, 21), 0);
        assert_eq!(func(14, 13), -1);
    }
    test(min_changes);
}
