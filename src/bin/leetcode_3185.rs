//! 构成整天的下标对数目 II

pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
    let mut result = 0;
    let mut cnt = [0; 24];
    for t in hours {
        result += cnt[(24 - t as usize % 24) % 24];
        cnt[t as usize % 24] += 1;
    }
    result
}

fn main() {
    fn test(func: fn(hours: Vec<i32>) -> i64) {
        assert_eq!(func(vec![12, 12, 30, 24, 24]), 2);
        assert_eq!(func(vec![72, 48, 24, 3]), 3);
    }
    test(count_complete_day_pairs);
}
