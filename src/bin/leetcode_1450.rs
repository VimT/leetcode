//! 在既定时间做作业的学生人数

pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    start_time.into_iter().zip(end_time).filter(|&(s, t)| query_time >= s && query_time <= t).count() as i32
}

fn main() {
    fn test(func: fn(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
        assert_eq!(func(vec![4], vec![4], 4), 1);
    }
    test(busy_student);
}
