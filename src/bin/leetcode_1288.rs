//! 删除被覆盖区间

pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable_by_key(|x| (x[0], -x[1]));
    let len = intervals.len();
    let mut prev_end = 0;
    let mut result = 0;
    for i in 0..len {
        if intervals[i][1] > prev_end {
            result += 1;
            prev_end = intervals[i][1];
        }
    }
    result
}

fn main() {
    assert_eq!(remove_covered_intervals(vec![vec![1, 2], vec![1, 4], vec![3, 4]]), 1);
    assert_eq!(remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]]), 2);
}

