//! 同时运行 N 台电脑的最长时间

pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
    let sum: i64 = batteries.iter().map(|x| *x as i64).sum();
    let mut left = 0;
    let n = n as i64;
    let mut right = sum / n + 1;
    while left < right {
        let mid = (left + right) / 2;
        let mut total = 0;
        for &cap in &batteries {
            total += mid.min(cap as i64);
        }
        // mid >= target { left = mid + 1; } 写法，当有重复元素时，返回最大的那个
        if total >= n * mid {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left - 1
}

fn main() {
    assert_eq!(max_run_time(12, vec![11, 89, 16, 32, 70, 67, 35, 35, 31, 24, 41, 29, 6, 53, 78, 83]), 43);
    assert_eq!(max_run_time(2, vec![3, 3, 3]), 4);
    assert_eq!(max_run_time(2, vec![1, 1, 1, 1]), 2);
}
