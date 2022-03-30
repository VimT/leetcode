//! 完成旅途的最少时间

pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
    let mut left = 0;
    let mut right = total_trips as i64 * *time.iter().min().unwrap() as i64;
    while left < right {
        let mid = (left + right) / 2;
        let mut trips = 0;
        for &t in &time {
            trips += mid / t as i64;
        }
        if trips >= total_trips as i64 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    assert_eq!(minimum_time(vec![1, 2, 3], 5), 3);
    assert_eq!(minimum_time(vec![2], 1), 2);
}
