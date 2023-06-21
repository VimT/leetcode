//! 在区间范围内统计奇数数目

pub fn count_odds(low: i32, high: i32) -> i32 {
    let mut result = (high - low) / 2;
    if low & 1 == 1 || high & 1 == 1 { result += 1; }
    result
}

fn main() {
    fn test(func: fn(low: i32, high: i32) -> i32) {
        assert_eq!(func(3, 7), 3);
        assert_eq!(func(8, 10), 1);
    }
    test(count_odds);
}
