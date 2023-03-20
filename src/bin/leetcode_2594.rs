//! 修车的最少时间

pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    let min_rank = *ranks.iter().min().unwrap();
    let mut left = 1;
    let cars = cars as u64;
    let mut right = min_rank as u64 * cars * cars;
    while left < right {
        let mid = (left + right) / 2;
        let mut can_cars = 0;
        for &rank in &ranks {
            can_cars += ((mid / rank as u64) as f64).sqrt() as u64;
        }
        if can_cars >= cars {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as i64
}

fn main() {
    fn test(func: fn(ranks: Vec<i32>, cars: i32) -> i64) {
        assert_eq!(func(vec![4, 2, 3, 1], 10), 16);
        assert_eq!(func(vec![5, 1, 8], 6), 16);
    }
    test(repair_cars);
}
