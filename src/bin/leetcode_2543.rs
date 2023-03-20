//! 判断一个点是否可以到达

use leetcode::gcd::gcd;

pub fn is_reachable(target_x: i32, target_y: i32) -> bool {
    gcd(target_x, target_y).count_ones() == 1
}

fn main() {
    fn test(func: fn(target_x: i32, target_y: i32) -> bool) {
        assert_eq!(func(6, 9), false);
        assert_eq!(func(4, 7), true);
    }
    test(is_reachable);
}
