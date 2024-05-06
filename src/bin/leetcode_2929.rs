//! 给小朋友们分糖果 II

pub fn distribute_candies(n: i32, limit: i32) -> i64 {
    let mut result = 0;
    for i in 0..=limit.min(n) {
        let left = n - i; // left 个糖果 2 个人分
        result += (left.min(limit) - (left - limit).max(0) + 1).max(0) as i64;
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, limit: i32) -> i64) {
        assert_eq!(func(5, 2), 3);
        assert_eq!(func(3, 3), 10);
    }
    test(distribute_candies);
}
