//! 给小朋友们分糖果 I

pub fn distribute_candies(n: i32, limit: i32) -> i32 {
    let mut result = 0;
    for i in 0..=limit.min(n) {
        let left = n - i; // left 个糖果 2 个人分
        result += (left.min(limit) - (left - limit).max(0) + 1).max(0);
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, limit: i32) -> i32) {
        assert_eq!(func(4, 1), 0);
        assert_eq!(func(1, 3), 3);
        assert_eq!(func(5, 2), 3);
        assert_eq!(func(3, 3), 10);
    }
    test(distribute_candies);
}
