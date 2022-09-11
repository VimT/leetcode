//! 分组的最大数量

pub fn maximum_groups(grades: Vec<i32>) -> i32 {
    let len = grades.len();
    (((len * 2) as f64 + 0.25).sqrt() - 0.5) as i32
}

fn main() {
    fn test(func: fn(grades: Vec<i32>) -> i32) {
        assert_eq!(func(vec![10, 6, 12, 7, 3, 5]), 3);
        assert_eq!(func(vec![8, 8]), 1);
    }
    test(maximum_groups);
}
