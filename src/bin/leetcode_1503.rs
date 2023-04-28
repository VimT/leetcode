//! 所有蚂蚁掉下来前的最后一刻

pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
    left.iter().max().copied().unwrap_or(0).max(n - right.iter().min().copied().unwrap_or(n))
}

fn main() {
    fn test(func: fn(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32) {
        assert_eq!(func(4, vec![4, 3], vec![0, 1]), 4);
        assert_eq!(func(7, vec![], vec![0, 1, 2, 3, 4, 5, 6, 7]), 7);
        assert_eq!(func(7, vec![0, 1, 2, 3, 4, 5, 6, 7], vec![]), 7);
    }
    test(get_last_moment);
}
