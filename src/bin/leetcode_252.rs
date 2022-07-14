//! 会议室

pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
    intervals.sort_unstable();
    for interval in intervals.windows(2) {
        if interval[0][1] > interval[1][0] {
            return false;
        }
    }
    true
}

fn main() {
    fn test(func: fn(intervals: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(vec![vec![0, 30], vec![5, 10], vec![15, 20]]), false);
        assert_eq!(func(vec![vec![7, 10], vec![2, 4]]), true);
    }
    test(can_attend_meetings);
}
