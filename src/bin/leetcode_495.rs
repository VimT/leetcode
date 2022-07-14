//! 提莫攻击

pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    if time_series.len() == 0 { return 0; }
    let mut result = duration;
    for i in 1..time_series.len() {
        if time_series[i] - time_series[i - 1] >= duration {
            result += duration;
        } else {
            result += time_series[i] - time_series[i - 1];
        }
    }
    return result;
}

fn main() {
    fn test(func: fn(time_series: Vec<i32>, duration: i32) -> i32) {
        assert_eq!(func(vec![1, 4], 2), 4);
        assert_eq!(func(vec![1, 2], 2), 3);
    }
    test(find_poisoned_duration);
}
