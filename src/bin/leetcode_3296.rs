//! 移山所需的最少秒数

pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
    let mut left = 1;
    let mut right = (mountain_height as i64 + 1) * mountain_height as i64 * worker_times[0] as i64 / 2;
    while left < right {
        let total_time = (left + right) / 2;
        let mut sum = 0;
        for &t in &worker_times {
            let a = t as f64;
            let b = t as f64;
            let c = 2. * total_time as f64;
            let discriminant = b * b + 4. * a * c;
            let x = (-b + discriminant.sqrt()) / (2. * a);
            sum += x as i32;
        }
        if sum >= mountain_height {
            right = total_time;
        } else {
            left = total_time + 1;
        }
    }
    left
}

fn main() {
    fn test(func: fn(mountain_height: i32, worker_times: Vec<i32>) -> i64) {
        assert_eq!(func(5, vec![3]), 45);
        assert_eq!(func(4, vec![2, 1, 1]), 3);
        assert_eq!(func(10, vec![3, 2, 2, 4]), 12);
        assert_eq!(func(5, vec![1]), 15);
    }
    test(min_number_of_seconds);
}
