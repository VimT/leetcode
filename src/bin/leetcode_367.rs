//! 有效的完全平方数

pub fn is_perfect_square(num: i32) -> bool {
    let mut left = 1;
    // 43340 = (i32::MAX as f64).sqrt() as i32
    let mut right = num.min(46340);
    while left < right {
        let mid = (left + right) / 2;
        if mid * mid >= num {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left * left == num
}

fn main() {
    fn test(func: fn(num: i32) -> bool) {
        assert_eq!(func(16), true);
        assert_eq!(func(14), false);
        assert_eq!(func(i32::MAX), false);
        assert_eq!(func(46340 * 46340), true);
    }
    test(is_perfect_square);
}
