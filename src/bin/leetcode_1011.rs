//! 在 D 天内送达包裹的能力

pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let mut left = *weights.iter().max().unwrap();
    let mut right: i32 = weights.iter().sum();
    while left < right {
        let mid = (left + right) / 2;
        let mut cur = 0;
        let mut cnt = 0;
        for &wei in &weights {
            if cur + wei > mid {
                cnt += 1;
                cur = 0;
            }
            cur += wei;
        }
        if cur > 0 { cnt += 1; }
        if cnt > days {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn main() {
    assert_eq!(ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
    assert_eq!(ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
    assert_eq!(ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
}
