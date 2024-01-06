//! 统计已测试设备

pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
    let mut result = 0;
    for num in battery_percentages {
        if num - result > 0 {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(battery_percentages: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 1, 2, 1, 3]), 3);
        assert_eq!(func(vec![0, 1, 2]), 2);
    }
    test(count_tested_devices);
}
