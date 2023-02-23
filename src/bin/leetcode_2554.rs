//! 从一个范围内选择最多整数 I

use std::collections::HashSet;

pub fn max_count(banned: Vec<i32>, n: i32, mut max_sum: i32) -> i32 {
    let set: HashSet<i32> = banned.into_iter().collect();
    let mut result = 0;
    for i in 1..=n {
        if !set.contains(&i) {
            max_sum -= i;
            result += 1;
        }
        if i > max_sum {
            break;
        }
    }
    result
}

fn main() {
    fn test(func: fn(banned: Vec<i32>, n: i32, max_sum: i32) -> i32) {
        assert_eq!(func(vec![1, 6, 5], 5, 6), 2);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6, 7], 8, 1), 0);
        assert_eq!(func(vec![11], 7, 50), 7);
    }
    test(max_count);
}
