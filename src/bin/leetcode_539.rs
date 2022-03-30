//! 最小时间差

use leetcode::svec;

pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    let len = time_points.len();
    let mut t = Vec::with_capacity(len);
    for i in time_points {
        t.push(i[..2].parse::<i32>().unwrap() * 60 + i[3..].parse::<i32>().unwrap());
    }
    t.sort_unstable();
    let mut result = t[0] + 24 * 60 - t[len - 1];
    for i in 1..len {
        result = result.min(t[i] - t[i - 1]);
    }
    result
}


fn main() {
    assert_eq!(find_min_difference(svec!["23:59","00:00"]), 1);
    assert_eq!(find_min_difference(svec!["00:00","23:59","00:00"]), 0);
}
