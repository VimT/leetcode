//! 最小处理时间

pub fn min_processing_time(mut processor_time: Vec<i32>, mut tasks: Vec<i32>) -> i32 {
    processor_time.sort_unstable();
    tasks.sort_unstable();
    tasks.reverse();
    processor_time.into_iter().enumerate().map(|(i, x)| {
        x + tasks[i * 4]
    }).max().unwrap()
}

fn main() {
    fn test(func: fn(processor_time: Vec<i32>, tasks: Vec<i32>) -> i32) {
        assert_eq!(func(vec![8, 10], vec![2, 2, 3, 1, 8, 7, 4, 5]), 16);
        assert_eq!(func(vec![10, 20], vec![2, 3, 1, 2, 5, 8, 4, 3]), 23);
    }
    test(min_processing_time);
}
