//! 无需开会的工作日

pub fn count_days(mut days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort_unstable_by_key(|x| x[0]);
    let mut last_end = 1;
    for meeting in meetings {
        if meeting[1] < last_end {
            continue;
        }
        days -= meeting[1] - last_end.max(meeting[0]) + 1;
        last_end = meeting[1] + 1;
    }
    days
}

fn main() {
    fn test(func: fn(days: i32, meetings: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(10, vec![vec![5, 7], vec![1, 3], vec![9, 10]]), 2);
        assert_eq!(func(5, vec![vec![2, 4], vec![1, 3]]), 1);
        assert_eq!(func(6, vec![vec![1, 6]]), 0);
    }
    test(count_days);
}
