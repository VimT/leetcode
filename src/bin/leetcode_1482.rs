//! 制作 m 束花所需的最少天数

pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    let mut left = 0;
    let max = bloom_day.iter().max().copied().unwrap();
    let mut right = max + 1;
    while left < right {
        let mid = (left + right) / 2;
        let mut cnt = 0;
        let mut can = 0;
        for &num in &bloom_day {
            if num > mid {
                can += cnt / k;
                cnt = 0;
            } else {
                cnt += 1;
            }
        }
        can += cnt / k;
        if can >= m {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if left > max { -1 } else { left }
}

fn main() {
    fn test(func: fn(bloom_day: Vec<i32>, m: i32, k: i32) -> i32) {
        assert_eq!(func(vec![1, 10, 3, 10, 2], 3, 1), 3);
        assert_eq!(func(vec![1, 10, 3, 10, 2], 3, 2), -1);
        assert_eq!(func(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
        assert_eq!(func(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6], 4, 2), 9);
    }
    test(min_days);
}
