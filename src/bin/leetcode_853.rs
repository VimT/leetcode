//! 车队

/// 单调栈
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut ps: Vec<(i32, f64)> = position.into_iter().zip(speed).map(|(pos, speed)| {
        (pos, (target - pos) as f64 / speed as f64)
    }).collect();
    ps.sort_unstable_by_key(|x| x.0);
    let mut s = vec![];
    for (_, cost) in ps {
        while !s.is_empty() && *s.last().unwrap() <= cost {
            s.pop().unwrap();
        }
        s.push(cost);
    }
    s.len() as i32
}

fn main() {
    assert_eq!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
    assert_eq!(car_fleet(10, vec![3], vec![3]), 1);
    assert_eq!(car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
}
