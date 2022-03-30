//! 爱吃香蕉的珂珂

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut left = 1;
    let mut right = *piles.iter().max().unwrap() as i64;
    while left < right {
        let mid = (left + right) / 2;
        let mut cost = 0;
        for &pile in &piles {
            cost += pile as i64 / mid + if pile as i64 % mid == 0 { 0 } else { 1 };
        }
        if cost > h as i64 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left as i32
}

fn main() {
    assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
}
